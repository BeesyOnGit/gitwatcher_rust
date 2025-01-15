# GitWatcher Rust (GwRB)

GitWatcher Rust (GwRB) is a lightweight and **FAST**, Rust-based tool designed to monitor changes in a Git repository, automatically pull updates, and deploy them to a server. It is ideal for continuous integration and deployment (CI/CD) workflows, ensuring your server is always up-to-date with the latest changes from your repository.

---

## Features

- **Automated Git Monitoring**: Tracks changes in a Git repository at a specified interval.
- **Automatic Pull and Deployment**: Pulls the latest changes and deploys them to the server.
- **Custom Build and Move Commands**: Executes custom build and deployment scripts.
- **Configurable**: Uses a `config.json` file to define repository, build, and deployment settings.
- **Lightweight and Fast**: Built in Rust for performance and efficiency.

---

## Installation

### For Linux Users

1. **Clone the Repository**:
   Clone the repository to get the pre-built binary and configuration file:
   ```
   git clone https://github.com/BeesyOnGit/GwRB-linux.git
   cd GwRB-linux
  ``

### Set Up the Configuration File:
Ensure the config.json file is in the same directory as the binary. Modify the config.json file to match your repository and deployment settings (see the Configuration section below).

2. **Make the Binary Executable**:
Grant execute permissions to the binary:

```bash
chmod +x gitwatcher_rust
```
3. **Run GitWatcher**:
Start the application by running:
```
./gitwatcher_rust
```

## Configuration

GitWatcher uses a config.json file to define the repository, build commands, deployment commands, and monitoring interval. Below is an example configuration:

Example config.json

```json
{
    "build": [
        "cd /home/gitwatcher_rust/project/Serveur && npm i && npm run build",
        "cd /home/gitwatcher_rust/project/Client && npm i && npm run build"
    ],
    "mouve": [
        "cp -r /home/gitwatcher_rust/project/Client/dist /var/www/project.com",
        "cp -r /home/gitwatcher_rust/project/Serveur/dist/* /var/www/api.project.com"
    ],
    "clear_folder": "/home/gitwatcher_rust/project",
    "repo": "https://github.com/exampleuser/example-repo.git",
    "interval_in_sec": "10"
}
```
Configuration Fields
repo: The URL of the Git repository to monitor. Replace https://github.com/exampleuser/example-repo.git with your repository URL.

*build**: A list of commands to execute for building the project.

*mouve**: A list of commands to execute for moving/deploying the built files.

*clear_folder*: The folder to clear before pulling new changes. This field is required.

*interval_in_sec*: The interval (in seconds) at which the repository is checked for updates.

## Usage

1. **Set Up the Configuration File**:
Ensure the config.json file is in the same directory as the binary and configured correctly.

2. **Run GitWatcher**:
Start the application by running:
```
./gitwatcher_rust
```

2. **Monitor Logs**:
GitWatcher will log its activities to the console


### it is recomanded to use a process manager to contain the program as for now it dosn't support to run on the background
