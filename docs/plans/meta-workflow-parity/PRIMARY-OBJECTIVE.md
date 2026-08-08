# PRIMARY OBJECTIVE — Meta-Workflow Generator Parity (2026-08-06)

**Source:** User directive 2026-08-06 (verbatim quote, lightly formatted for readability).

---

## Objective

Three natural-language prompts that resemble how the user engages with opencode chat sessions. Multiple agentic elements but written in natural language. May have JSON pasted at bottom for context — but not in the prompt instructions. Those are just large text paragraphs. Very similar to the prompts the user sends in opencode chat.

The user does NOT use markdown notation in their own prompts. They just say things, ask for different things to be correlated in different ways, ask for different types of research to be done, ask for different files to be summarized in different ways and hierarchies of folders, ask for certain things to be run with the CLI, ask for certain types of edits of files, edits of code projects or file-suite projects.

**Completion criteria:**

Three natural-language prompts based on the user's prompts in opencode chat session history (read directly from files using a skill to help do this iteratively). These must be natural language like the directive prompt itself.

Once all three natural-language prompts that resemble how the user engages with opencode are drafted:

1. Use those to live-system-test
2. Prove that BOTH the output workflow from each of the three prompts AND the execution of those output workflows produces the user's desired effect and results in reality in the exact desired way
3. Extensive logging reviewed to confirm both the output and behavior match expectations
4. Achieve incremental objectives along the way

The workflow execution engine must function as expected and efficiently through the framework enhancements needed to be efficient and produce expected results in reality. Use of smaller, more incremental task execution for local models must also be used.

If they're not able to accomplish things in the way we expect right off the bat, then we want virtuous loops that get the LLM to a level of exactness for each of the subtasks. Don't necessarily require perfection at every step — but at certain pinch points there is validation. These pinch points help keep the general meta-workflow generator executing properly with all three prompts.

When running through these three natural-language prompts, the agent must prove the results with real analysis and reasoning and thought about the reality of the results.

## Prompt Structure

- **2 complex prompts** — used as regression tests while iterating on the simple one
- **1 simple prompt (LAST one iterated on)** — control to prevent simple prompts from taking a long time to run through the meta-workflow generator. Goal: generator must know when to skip excessive validation on simple prompts and not iterate so much.

## Failure Recovery

Near-flawless execution with local model on all three natural-language prompts is NOT guaranteed. Example: this directive prompt itself describes agentic instructions (different files, web research via curl to DuckDuckGo, etc.) — if those are not working well, iterate on subsections of the `.yml` workflow meta-workflow generator one by one. Only run those sub-workflows. Try to iterate on them quickly as individual sub-objectives that have beginning and end pinch points — but ALSO incremental pinch points for validation that confirm everything is working as expected and passing what is needed before moving on to the next step.

If live system testing proves we are not where we need to be, then extensive debug logs + iterative fixes until objective is fully complete.

## Process Mandates

- **Iterate and ask questions** until user says each specific element is working
- **Regression testing** added for features and workflow input set
- **Logging level set** in workflow
- **TDD hard rule** (see AGENTS.md): run minimal tests, fail-first, then write code to pass
- **Schema discipline** (see AGENTS.md): strict rules, always adhere before executing
- **Code organization**: vertical slices with exploded architecture. Don't massively expand scope of framework. Iterate on meta-workflow generator.
- **Pause and ask** when stuck on a real user-decision choice (interface/behavior). Use multiple-choice question format.

## Source Prompt (verbatim, unedited)

> So we need three human language-sounding prompts that I could see myself potentially writing in open code that have multiple agentic elements but are still written in natural language for the most part and maybe at the most have copied and pasted Json for context at the bottom kind of a thing but not the prompt instructions those will those would just be in large text paragraphs graphs It's very similar to the problem the prompts that I send in open code here code.
>
> I don't add Mark down notation for my own prompts. I just say things and ask for different things to be correlated in different ways and for different types of research to be done and for different files to be summarized in different ways and different hierarchies of folders and for certain things to be run with the CLI and Certain types of edits of files and certain edits of code projects or file suite projects.
>
> The completion criteria is I only want three human readable prompts that are based on my prompts in open code chat session history read directly from files using a skill to make to help you do this iteratively. These must be natural language like this prompt here. Once all three natural language prompts that resemble how I engage with open code, Then we use those to live system test and prove that the Both the output workflow from each of the three prompts and the execution of this output workflows produces users the desired effect and results in reality in the exact desired way with extensive logging being reviewed to confirm both the output and the behavior matches what we expect and achieved the incremental objectives along the way.
>
> I want the workflow execution engine to function as expected and efficiently through both the frame of framework enhancements that are needed be be efficient and produce expected results in reality. The use of smaller more incremental task execution for the for these local models must also be used. If they're not able to a cause accomplish things in the way we expect right off the bat off the back then we want virtuous loops that get the LLM to a level of exactness for each of the subtasks. Also, we don't necessarily require perfection at every step but at certain pinch points there is validation and these pinch points help keep the general metal workflow generator executing properly with all three of these prompts. When you run through these three natural language prompts that you can approve yourself the results with real analysis and reasoning and thought of the reality of the results.
>
> Just make sure you follow my instructions in this prompt though making a file of this prompt in a place and making it your source of through of you objective which until complete you must iterate and ask question until I say each specific element is working and regression testing is added for the features and the workflow in the input set logging level in the workflow.
>
> Also make one of the 3 prompts and the last one you iterate on simple to as a control to prevent a simple prompt from taking a long time to run through the metaworkflow generator and use the other 2 prompts as regression tests as you iterate on the final prompt to make it know when to skip to validation on things and not iterate so much on simple prompts.
>
> We might not have a near flawless execution with the local model with all three natural language prompts. For example this prompt itself that describes agentic instructions of how to do what I'm asking for like how I want different files, or to Look at different web research sources using curl request to duckgo. If all the of the things I described are not working well then we need to iterate on subsections of the .yml workflow meta workflow generator one by one, only running those sub-work flows and trying to iterate on them quickly as individual sub-objectives that have beginning and end pinch points but also incremental pinch points for validation that confirm everything is working as expected and passing what is needed before moving on to the next step.
>
> If live system testing proves that this is not where we're at then we need extensive debug logs and we need to fix it iteratively until this objective is fully complete. Put in your agents.md top level main objective and that is our main goal is to accomplish in this repo. this objective where we have three natural language prompts that are very similar to my agentic natural language prompts that I send in open code and the result resulting workflow and the result of its own execution to pass or are equivalent to the performance that I get and reliability that I get in open code with tasks that the framework at the framework supports.
>
> Don't massively expand the scope of the framework but iterate on the meta workflow generator in long and log in the framework code and make sure that you are organizing the code in version in vertical slices with exploded architecture And make sure you iterate on to iterate on code with test-driven development where you run the minimal amount of tests as you are trying to accomplish things and you make a and you make sure the test fails before you write the code to make it pass And that also needs to be in the agents.md as a hard rule that we always follow when executing on the framework code to code Also add some notes and some notes about ways in which We iterate on workflows and workflow files and the strict rules around schema and how we follow those And always it and always adhere to those before executing on any of this.
>
> Continue iterating and if you get stuck stock with a real choice that a user really the user really would want to make about the interface or how something functions from a behavior standpoint in the framework or the workflow then pause and add and ask whatever questions you need and follow the instructions you use to generate this set of questions always when you ask questions. don't stop until it's accomplished or your positive or you're pausing to ask questions with open-code multiple- code multiple choice questions that are high quality in the way I've described before.
