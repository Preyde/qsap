# Third Party Command Line Interface for SAP

## Table of Content

1. [Introduction](#introduction)
2. [Installation](#Installation)
3. [Configuration](#Configuration)
4. [Usage](#usage)
5. [Platforms](#Platforms)
6. [Releases](#releases)
7. [Plans for Future](#Plans-for-Future)

# Introduction

This Project is a Command Line Interface written in Rust, where you can interact with your SAP-Systen. This is not a full-featured client, rather an additional tool you can use alongside your IDE while developing.

Currently the project is in an early phase and supports things like small operations on the data dictionary, displaying the content of a database table and copying a database table to another system.

# Installation

The executable can be download from GitHub.

To do that click the following URL: ""


# Configuration

When you downloaded the executable, open the terminal and go to the directory that currently contains the executable (probably the download folder). 

You can move it to your home directory by entering:
```
mv sap_cli $Env:UserProfile
```

Now you can start the application for the first time. By entering the name of the executable (sap_cli) in the command line, the application starts and creates the nessesary files. 

## Connecting to your sap-system

To connect the application to your sap-system, you need to provide the server details in the destination file. To open it in your default editor type the following command:

```
sap_cli dest
```

The destination file with an empty entry should open and has the following structure:

```json
[
   {
    "sys_id": "ITK", 
    "host": "https://example-system-itk.com/",    
    "port": 1234,  
    "uname": "USER123",  
    "passwd": "password123",  
    "mandt": "123", 
    "lang": "EN"
  },
]
```

Fill out those 7 fields. To configure multiple system connections just add an addition entry between the brackets like below:
```json
[
  {
    "sys_id": "ITK", 
    "host": "https://example-system-itk.com/",    
    "port": 1234,  
    "uname": "USER123",  
    "passwd": "password123",  
    "mandt": "123", 
    "lang": "EN"
  },
    {
    "sys_id": "AEI", 
    "host": "http://system-aei.com", 
    "port": 1234,
    "uname": "USER123", 
    "passwd": "password123",
    "mandt": "200", 
    "lang": "DE" 
  }
]
```

When you now run the application again using
```
sap_cli
```

you will get asked to encrypt the passwords. If you press enter you can provide a master password which will be used to encrypt and decrypt all the system passwords passwords. Your master password is NOT saved. If you don't remember it, just open the destination file and provide the system password again.

To test your connected systems run the following command:

```
sap_cli check
```
After you provided your master password, you should get a green checkmark as response.

## Setting your default system

To set a default system you want to work with run the following command:

```
sap_cli settings -s "{sys_id}"
```
The sys_id in the command is the one you put in your destination file.


```
sap_cli settings -s "KTI"
```

Now the configuration is done and you can start playing around.

# Usage

There will be an online list to all the possibilitys in the future. For the first you can run the application without subcommands to get the help list.

```
sap_cli --help
```
or
```
sap_cli
```

Some things you can do with this tool is for example:

- Get the content of a database table

    ```
    sap_cli tab {Name of database table} -r 50
    ```
- Create a development object

    ```
    sap_cli new prog {Name of the program}
    ```

- Copy a database table from one system to another

    ```
    sap_cli copy tab {Name of the table to copy} -d {sys_id of the destination to copy to}
    ```

To include your objects in a transport request, many commands have the option -t where you can pass it. When you want to write it into a transprot request you also need to pass a package name with -p.

# Platforms

The Application is currently built for Windows only.


# Releases

Patches including new features and bug fixes will come weekly.

# Plans for Future

In future releases the application should enable copy, change and delete operation for all dictionary objects. 

Also planned is the introduction of git and to make the sharing of programs and ddic objects easier through the client.


