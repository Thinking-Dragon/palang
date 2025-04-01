# palang-debugger README
## Features
### Task execution
The extension presents the user with all the prompts and functions (collectively refered to as tasks) present in your VS Code workspace.
The user can run a task by clicking on the "Run" button beside it.
If the task has parameters, then the user is presented with input fields to provide values for the parameters before running the task.

### Live compilation
When you save a file, the extension automatically compiles it and updates the list of tasks it presents to the user.

### Profile selection
In the top section of the extension, the user is presented with a selector to choose one of the Palang profiles present in the current Palang context.

## Requirements
For this extension to function, you must install the `palang` CLI on your machine.
On an Ubuntu machine: `snap install palang`.

## Extension Settings
None.

## Known Issues
None so far.

## Release Notes
### 1.0.0
Initial release of the Palang Debugger VS Code extensithe Palang Debugger VS Code extension.
Implemented task execution, live compilation and profile selection.
