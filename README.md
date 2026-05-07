# yy-glob-run
Execute command with expanded glob

## Usage on Single mode
```
gr s <options> <command including glob>
```

### Options
-p (number of threads): Specify the number of threads to use for parallel execution. default: number of CPU threads

## Usage on Extend mode
```
gr x <options> <command including glob>
```
### Options
-p (number of threads): Specify the number of threads to use for parallel execution. default: number of CPU threads
-m (max depth): Specify the maximum extend glob. default: 128
