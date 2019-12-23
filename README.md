# Asana
 Asana command line interface

### Usage
```
asana --help

USAGE:
    asana [FLAGS] [OPTIONS] --task <task_id>

FLAGS:
    -h, --help       Prints help information
        --tasks      Get all tasks
    -V, --version    Prints version information

OPTIONS:
    -c, --comment <comment>    Add comment on a task
    -f, --finish <finish>      Complete/Uncomplete a Task. Accepts true or false
    -n, --note <note>          Add note on a task
    -t, --task <task_id>       Get Specific Task
```

### Config File

 * In the same directory as your executable.
 * Must be named "config.conf"
 * `TOKEN`=`your personnal access token`
 * `USER_GID`=`your GID user` => use to get all your tasks
 * `WORKSPACE`=`WORKSPACE ID` => Define the default workspace. Can be override by --workspace "Id workspace" or -w "id workspace"
 
### TODO 

 * [OK] Post comment
 * Assigne to someone
 * Create a Task
 * Link to a project
 * [OK] List all workspaces
 * [OK] List All Projects in workspace
 * [OK] View all tasks in a project
 * Refacto Api when I have more skills in Rust :D
 * Set a deadline
 * Custom Error Message
