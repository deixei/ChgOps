# Ideas for this product

- It must allow to init a new project that will have many playbooks

- It must allow to run playbooks

- It must have a build process to create a deployable unit

- It must have a testing harness to run test cases against playbooks


## possible Commands 


chgops init --name az_deixei --template basic

chgops run --playbook az_deixei/playbook.yml --verbose vvv --arguments 'ARG1=1 ARG2=3'

chgops build --debug true --change_id 1234

chgops test --scope p1

chgops publish --ado_pack?? --package az_deixei-1_1_0-1234.zip

chgops download --name az_deixei --version latest


## Possible fetures of a playbook

- Read env vars

- load vars from files

- load vars from config sources (CosmosDB, APIs)

- load a collection

- depend on a collection

- run a task

- run a block of tasks

- run a for each block of tasks

- pass variables to a task

- task output is named

- debug tasks

- display tasks

- accert tasks

- set values tasks

- variables value manipulations (call functions)

- invoke a task in another file


## Software Arch ??
playbook
    mod.rs
    models.rs
    run.rs
    tasks.rs
    vars.rs
    blocks.rs
    foreach.rs
    
handlers
    init.rs

Commands
    mod.rs
    run.rs
    init.rs

main.rs


### dependencies

Serge for yaml
clap for CLI parsing

https://azure.github.io/azure-sdk/rust_introduction.html

