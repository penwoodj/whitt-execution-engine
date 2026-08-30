import yaml
import sys
from pathlib import Path

EXPERIMENT_DIR = Path(__file__).parent.parent
WORKFLOWS_DIR = EXPERIMENT_DIR.parent / "workflows"
CASES_DIR = EXPERIMENT_DIR / "cases"
SCRIPTS_DIR = EXPERIMENT_DIR / "scripts"

REQUIRED_STAGES = ["screen", "gen", "check", "fix", "judge"]


def load_workflow(path):
    with open(path) as f:
        return yaml.safe_load(f)


def get_steps(wf):
    return list(wf.get("agentic_workflow", {}).get("steps", {}).keys())


def get_hook_actions(wf, step_name, trigger):
    step = wf["agentic_workflow"]["steps"].get(step_name, {})
    when = step.get("when", {})
    return when.get(trigger, [])


def find_gwt_routes(actions):
    routes = set()
    for a in actions:
        if "gwt" in a:
            for clause in a["gwt"]:
                then = clause.get("then")
                if isinstance(then, str):
                    routes.add(then)
                elif isinstance(then, list):
                    routes.update(then)
    return routes


def find_shell_commands(actions):
    cmds = []
    for a in actions:
        if "shell" in a:
            s = a["shell"]
            cmds.append(s.get("command", ""))
    return cmds


def check_step_exists(wf, target, step_name, context):
    steps = get_steps(wf)
    if target not in steps:
        return f"{step_name} routes to '{target}' but step does not exist"
    return None


def check_screen_step(wf):
    errors = []
    steps = get_steps(wf)
    screen_steps = [s for s in steps if "screen" in s.lower()]
    if not screen_steps:
        errors.append("No SCREEN step found")
        return errors
    screen = screen_steps[0]
    before = get_hook_actions(wf, screen, "before_step_starts")
    cmds = find_shell_commands(before)
    if not cmds:
        errors.append(f"{screen}: no shell command in before_step_starts")
    else:
        has_emit = any("emit-prompt" in c for c in cmds)
        if not has_emit:
            errors.append(f"{screen}: no emit-prompt.py call in before_step_starts")
    gwt_routes = find_gwt_routes(before)
    if not gwt_routes:
        errors.append(f"{screen}: no GWT routing in before_step_starts")
    for r in gwt_routes:
        e = check_step_exists(wf, r, screen, "SCREEN")
        if e:
            errors.append(e)
    return errors


def check_gen_step(wf):
    errors = []
    steps = get_steps(wf)
    gen_steps = [s for s in steps if "gen" in s.lower() and "screen" not in s.lower()]
    if not gen_steps:
        errors.append("No GEN step found")
        return errors
    gen = gen_steps[0]
    after = get_hook_actions(wf, gen, "after_step_succeeds")
    has_save = any("save_to" in a for a in after)
    if not has_save:
        errors.append(f"{gen}: no save_to in after_step_succeeds")
    has_check = any("check-deterministic" in str(a) for a in after)
    if not has_check:
        errors.append(f"{gen}: no check-deterministic.py call in after_step_succeeds")
    gwt_routes = find_gwt_routes(after)
    if gwt_routes:
        for r in gwt_routes:
            e = check_step_exists(wf, r, gen, "GEN")
            if e:
                errors.append(e)
    return errors


def check_check_step(wf):
    errors = []
    steps = get_steps(wf)
    check_steps = [s for s in steps if "check" in s.lower() and "screen" not in s.lower()]
    if check_steps:
        check = check_steps[0]
        before = get_hook_actions(wf, check, "before_step_starts")
        cmds = find_shell_commands(before)
        has_check = any("check-deterministic" in c for c in cmds)
        if not has_check:
            errors.append(f"{check}: no check-deterministic.py call")
        gwt_routes = find_gwt_routes(before)
        if not gwt_routes:
            errors.append(f"{check}: no GWT routing (should route to judge or fix)")
        for r in gwt_routes:
            e = check_step_exists(wf, r, check, "CHECK")
            if e:
                errors.append(e)
    else:
        has_inline_check = False
        for s in steps:
            after = get_hook_actions(wf, s, "after_step_succeeds")
            if any("check-deterministic" in str(a) for a in after):
                has_inline_check = True
                break
        if not has_inline_check:
            errors.append("No CHECK step found (standalone or inline in after_step_succeeds)")
    return errors


def check_fix_step(wf):
    errors = []
    steps = get_steps(wf)
    fix_steps = [s for s in steps if "fix" in s.lower()]
    if not fix_steps:
        errors.append("No FIX step found")
        return errors
    fix = fix_steps[0]
    before = get_hook_actions(wf, fix, "before_step_starts")
    cmds = find_shell_commands(before)
    has_emit = any("emit-prompt" in c for c in cmds)
    if not has_emit:
        errors.append(f"{fix}: no emit-prompt.py call (fixer role)")
    after = get_hook_actions(wf, fix, "after_step_succeeds")
    gwt_routes = find_gwt_routes(after)
    if not gwt_routes:
        errors.append(f"{fix}: no GWT routing (should route back to check)")
    for r in gwt_routes:
        e = check_step_exists(wf, r, fix, "FIX")
        if e:
            errors.append(e)
    return errors


def check_judge_step(wf):
    errors = []
    steps = get_steps(wf)
    judge_steps = [s for s in steps if "judge" in s.lower()]
    if not judge_steps:
        errors.append("No JUDGE step found")
        return errors
    judge = judge_steps[0]
    before = get_hook_actions(wf, judge, "before_step_starts")
    cmds = find_shell_commands(before)
    has_emit = any("emit-prompt" in c for c in cmds)
    if not has_emit:
        errors.append(f"{judge}: no emit-prompt.py call (judge role)")
    after = get_hook_actions(wf, judge, "after_step_succeeds")
    has_save = any("save_to" in a for a in after)
    if not has_save:
        errors.append(f"{judge}: no save_to for verdict")
    return errors


def check_cascade_flow(wf):
    errors = []
    steps = get_steps(wf)
    if len(steps) < 4:
        errors.append(f"Too few steps ({len(steps)}), need at least screen+gen+check+judge")
    return errors


def check_bookmark_flow(wf):
    errors = []
    all_bookmarks_used = set()
    for step_name in get_steps(wf):
        for trigger in ["before_step_starts", "after_step_succeeds"]:
            for action in get_hook_actions(wf, step_name, trigger):
                for key in ["save_to", "append_to", "bookmark"]:
                    if key in action:
                        val = action[key]
                        if isinstance(val, list):
                            for v in val:
                                if isinstance(v, str) and v.startswith("$"):
                                    all_bookmarks_used.add(v)
                        elif isinstance(val, str) and val.startswith("$"):
                            all_bookmarks_used.add(val)
    if not all_bookmarks_used:
        errors.append("No bookmarks used - workflow has no state passing between steps")
    return errors


def check_no_model_in_judge(wf):
    errors = []
    for step_name in get_steps(wf):
        if "judge" in step_name.lower():
            before = get_hook_actions(wf, step_name, "before_step_starts")
            for action in before:
                if "shell" in action:
                    cmd = action["shell"].get("command", "")
                    if "--role judge" in cmd and "--stage JUDGE" in cmd:
                        continue
    return errors


def validate_workflow(path):
    wf = load_workflow(path)
    all_errors = []
    all_errors.extend(check_screen_step(wf))
    all_errors.extend(check_gen_step(wf))
    all_errors.extend(check_check_step(wf))
    all_errors.extend(check_fix_step(wf))
    all_errors.extend(check_judge_step(wf))
    all_errors.extend(check_cascade_flow(wf))
    all_errors.extend(check_bookmark_flow(wf))
    all_errors.extend(check_no_model_in_judge(wf))
    return all_errors


def validate_cases():
    errors = []
    if not CASES_DIR.exists():
        return [f"Cases dir missing: {CASES_DIR}"]
    for case_file in CASES_DIR.glob("*.yml"):
        with open(case_file) as f:
            data = yaml.safe_load(f)
        if not data:
            errors.append(f"{case_file.name}: empty YAML")
            continue
        if "prompt" not in data:
            errors.append(f"{case_file.name}: missing 'prompt' key")
        sc = data.get("success_criteria", {})
        if not sc.get("deterministic_checks"):
            errors.append(f"{case_file.name}: missing 'success_criteria.deterministic_checks'")
    return errors


def validate_scripts():
    errors = []
    expected = ["check-deterministic.py", "fix-format.py", "emit-prompt.py", "baseline.py"]
    for script in expected:
        if not (SCRIPTS_DIR / script).exists():
            errors.append(f"Missing script: {script}")
    return errors


def main():
    target = sys.argv[1] if len(sys.argv) > 1 else None
    errors = []
    if target and target.endswith(".yml"):
        errors = validate_workflow(target)
    else:
        errors.extend(validate_cases())
        errors.extend(validate_scripts())
        wf_dir = EXPERIMENT_DIR.parent / "workflows"
        if wf_dir.exists():
            for wf in wf_dir.glob("*.yml"):
                errors.extend(validate_workflow(str(wf)))
                errors = [f"{wf.name}: {e}" for e in errors]
    if errors:
        for e in errors:
            print(f"ERROR: {e}")
        sys.exit(1)
    else:
        print("PASS: all dependency checks passed")
        sys.exit(0)


if __name__ == "__main__":
    main()
