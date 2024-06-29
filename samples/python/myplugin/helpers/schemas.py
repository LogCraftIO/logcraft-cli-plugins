# Copyright (c) 2023 LogCraft, SAS.
# SPDX-License-Identifier: MPL-2.0
import os

def generate():
    """
    Convert .k files from the 'package' directory to .py files
    """
    root = os.path.join(os.path.dirname(__file__), '../')
    root = os.path.abspath(root)

    # schemas directory
    schemas_dir = os.path.join(root, 'schemas')
    if not os.path.exists(schemas_dir):
        os.makedirs(schemas_dir)
    
    # convert .k files to .py files
    for filename in ["settings.k", "rule.k"]:
        filepath = os.path.join(root, 'package', filename)
        filepath = os.path.abspath(filepath)

        with open(filepath, 'r') as f:
            content = f.read()

        variable = filename.replace('.k', '')
        f_out = os.path.join(schemas_dir, filename.replace('.k', '.py'))
        with open(f_out, 'w') as f:
            f.write("%s = '''%s'''" % (variable, content))


if __name__ == "__main__":
    generate()
