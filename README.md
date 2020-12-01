# Stratosphere
`Stratosphere` is a fully open source Social Network backend written in Rust, inspired by the likes of Twitter and GooglePlus.

## Features
For all information regarding progress on updates or features, please have a look at our
[Milestones](https://github.com/UndefinedBHVR/Stratosphere/milestones) or [Project Boards](https://github.com/UndefinedBHVR/projects/)
## Installation
To install Atomic, all you need to do is the following:

First, clone the project.
```bash
git clone git@github.com:UndefinedBHVR/Stratosphere.git
```
Secondly, using the `example.env` file as a template, create your own `.env`, altering information where necessary.

Ex: 
```dotenv
DATABASE_URL=postgres://stratosphere:atmosphere@localhost/db
```
Next, assuming you have `diesel_cli`installed, run the following:
```shell script
diesel migration run
```
In order to create the database and include all the required tables.\
Afterwards, all that's left is to compile and run.
## Contributing
Pull requests are always welcomed. If you have a potentially breaking change, please open an issue first stating what the change is.

Please also make sure to add tests, or update old ones if required!

## License
Statosphere is currently licensed under the MIT License.\
[MIT](https://choosealicense.com/licenses/mit/)