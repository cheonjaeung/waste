# Waste

A simple command line tool to move files and directories to the trash,
Uses native APIs or standard specifications to move files to the system's default trash or recycle bin.

> [!NOTE]
> This tool currently supports only macOS.
> Windows and Linux are planned.

## Usage

Waste is a simple command line tool.
You can remove a file or directory just like this.

```shell
waste <PATH>
```

You can move multiple files or directories to the trash.

```shell
waste uselessfile1.txt uselessfile2.txt uselessdir
```

Print help for more information.

```shell
waste --help
```

## License

This project is licensed under the Apache License 2.0.

```
Copyright 2026 Cheon Jaeung

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
