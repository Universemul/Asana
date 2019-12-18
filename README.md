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
 
### TODO 

 * [OK] Post comment
 * Set a deadline
 * Assigne to someone
 * Create a Task
 * Link to a project
 * View all tasks in a project
 * Refacto Api when I have more skills in Rust :D
