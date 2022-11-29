# hello_rust

# CodeLLDB crash

[link](https://github.com/vadimcn/vscode-lldb/issues/410)

- In LLDB's output I was getting `Debug adapter exit code=3221225620` at the last line.
- In console output, I was just getting `No connection could be made because the target machine actively refused it.`
- As a popup, I got `Oops! The debug adapter has terminated abnormally.`

```cmd
regsvr32.exe %USERPROFILE%\.vscode\extensions\vadimcn.vscode-lldb-1.8.1\adapter\msdia140.dll
```
