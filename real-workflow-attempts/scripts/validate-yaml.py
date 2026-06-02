#!/usr/bin/env python3
"""Validate a YAML workflow file against basic structural requirements."""
import sys
import yaml
import json

def validate(filepath):
    errors = []
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        # Strip markdown fences if present
        if content.strip().startswith('```'):
            lines = content.strip().split('\n')
            # Remove first line (```yaml or ```) and last line (```)
            if lines[0].startswith('```'):
                lines = lines[1:]
            if lines and lines[-1].strip() == '```':
                lines = lines[:-1]
            content = '\n'.join(lines)
        
        # Parse YAML
        data = yaml.safe_load(content)
        
        if not isinstance(data, dict):
            errors.append("Root must be a mapping/dict")
            return errors
        
        # Required fields
        required = ['workflow_id', 'name', 'providers', 'models', 'agentic_workflow']
        for field in required:
            if field not in data:
                errors.append(f"Missing required field: {field}")
        
        # Provider validation
        if 'providers' in data and isinstance(data['providers'], dict):
            for pname, pval in data['providers'].items():
                if not isinstance(pval, dict):
                    errors.append(f"Provider '{pname}' must be a mapping")
                    continue
                if 'config' not in pval and 'config_file' not in pval:
                    errors.append(f"Provider '{pname}' missing 'config' or 'config_file'")
                if 'config' in pval and isinstance(pval['config'], dict):
                    if 'host' not in pval['config']:
                        errors.append(f"Provider '{pname}' config missing 'host'")
                    if 'port' not in pval['config']:
                        errors.append(f"Provider '{pname}' config missing 'port'")
        
        # Model validation
        if 'models' in data and isinstance(data['models'], dict):
            for mname, mval in data['models'].items():
                if not isinstance(mval, dict):
                    errors.append(f"Model '{mname}' must be a mapping")
                    continue
                if 'name' not in mval:
                    errors.append(f"Model '{mname}' missing 'name'")
                if 'host' not in mval:
                    errors.append(f"Model '{mname}' missing 'host'")
                elif isinstance(mval['host'], dict) and 'type' not in mval['host']:
                    errors.append(f"Model '{mname}' host missing 'type'")
        
        # Agentic workflow validation
        if 'agentic_workflow' in data and isinstance(data['agentic_workflow'], dict):
            if 'steps' not in data['agentic_workflow']:
                errors.append("agentic_workflow missing 'steps'")
            elif isinstance(data['agentic_workflow']['steps'], dict):
                for sname, sval in data['agentic_workflow']['steps'].items():
                    if not isinstance(sval, dict):
                        errors.append(f"Step '{sname}' must be a mapping")
                        continue
                    if 'generative_entity' not in sval and 'gwt' not in sval.get('when', {}):
                        # Control flow steps don't need generative_entity
                        if 'when' not in sval:
                            errors.append(f"Step '{sname}' missing 'generative_entity' or 'when'")
        
        # Check for invalid top-level keys (basic schema compliance)
        valid_top_keys = {
            'workflow_id', 'name', 'description', 'version', 'author', 'tags',
            'min_schema_version', 'schema_version', 'providers', 'models',
            'agentic_workflow', 'workflow_execution_strategy', 'when',
            'sub_workflows', 'tool_permissions', 'memory', 'workspace'
        }
        for key in data:
            if key not in valid_top_keys:
                errors.append(f"Unknown top-level key: '{key}' (may not be schema-compliant)")
        
        if not errors:
            print(f"VALID: {filepath}")
            return []
        else:
            for e in errors:
                print(f"ERROR: {e}", file=sys.stderr)
            return errors
    
    except yaml.YAMLError as e:
        error_msg = f"YAML parse error: {e}"
        print(f"FATAL: {error_msg}", file=sys.stderr)
        return [error_msg]
    except FileNotFoundError:
        error_msg = f"File not found: {filepath}"
        print(f"FATAL: {error_msg}", file=sys.stderr)
        return [error_msg]
    except Exception as e:
        error_msg = f"Unexpected error: {e}"
        print(f"FATAL: {error_msg}", file=sys.stderr)
        return [error_msg]

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: validate-yaml.py <filepath>", file=sys.stderr)
        sys.exit(1)
    errors = validate(sys.argv[1])
    sys.exit(len(errors))
