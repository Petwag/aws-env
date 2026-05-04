This is a simple cli to fetch env from ECS task definition or Lambda function and write it down in a .env format to be ready to use

# Installation
## Prerequisite
You need to use the aws cli to login to a profile locally on your computer.
You then need to login to the account you want to use.
For example if you have sso configured
```bash
aws sso login --profile Foo
```

## Executable
Download the .exe file and add the path to your Env Variables.

## Build
Clone that repo and build the code 
```bash
cargo build --release
```
Then add the .exe file from ./target/release to your Env Variables

# Usage
## Help
```bash
aws-env --help
```

## Basic usage
```bash
aws-env
```

Then answer the questions. 
The profile you use should be already logged in.

## Params
- --profile, -p : This is the aws profile that will be used to fetch infos
- --output, -o : This is the name of the output file the cli will write to
- --what, -w : This is the type of ressource you want to target.

### Ressource : ECS
- --task-definition, -t : The task definition name.

### Ressource : Lambda
- --lambda, -l : The lambda function name

## TODO
- Adding an arg to add a valid AccessToken to the output file + AWS_REGION if needed
- Adding support for env override in case local dev has different requirements.
- Remove the nasty unwrap and handle errors in a cleaner way than just exploding