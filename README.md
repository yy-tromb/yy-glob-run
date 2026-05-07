# yy-***g***lob-***r***un
Execute command with expanded glob.  
This is useful for Windows Command Prompt.

## Usage on Single mode
```
gr s <options> <command including glob>
```
### Examples
- `gr s ffprobe -hide_banner *.mp4`
- `gr s -p8 cmd /c type *.csv` (stdout is not controled, so this example may be mixed in line.)

### Options
-p(number of threads): Specify the number of threads to use for parallel execution. default: number of CPU threads

## Usage on Extend mode
```
gr x <options> <command including glob>
```
### Options
-p(number of threads): Specify the number of threads to use for parallel execution. default: number of CPU threads
-m(max length): Specify the maximum extending glob. default: 256
