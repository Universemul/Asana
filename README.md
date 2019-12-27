# Asana
 Asana command line interface

### Usage
```
asana --help

USAGE:
    asana [FLAGS] [OPTIONS]

FLAGS:
    -h, --help          Prints help information
        --projects      Display all Projects
        --tasks         Display all tasks
        --users         Display all users
    -V, --version       Prints version information
        --workspaces    Display all workspace

OPTIONS:
    -a <assignee>              Assignee to a user. Can be Gid or Name
    -c, --comment <comment>    Add comment on a task
    -f, --finish <finish>      Complete/Uncomplete a Task. Accepts true or false
    -n, --note <note>          Add note on a task
    -p <project_id>            Specify a project
    -t <task_id>               Specify a task
    -w <workspace_id>          Specify a workspace
```

### Config File

 * In the same directory as your executable.
 * Must be named "config.conf"
 * `TOKEN`=`your personnal access token`
 * `USER_GID`=`your GID user` => use to get all your tasks
 * `WORKSPACE`=`WORKSPACE ID` => Define the default workspace. Can be override by --workspace "Id workspace" or -w "id workspace"
 
### TODO 

 * [OK] Post comment
 * [OK] Assignee to someone by id
 * [OK] Assignee to someone by name
 * Create a Task
 * [OK] Link to a project
 * [OK] List all workspaces
 * [OK] List All Projects in workspace
 * [OK] View all tasks in a project
 * Refacto Global
 * Set a deadline
 * Custom Error Message
