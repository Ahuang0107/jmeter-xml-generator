# Readme

- serve doc
    - `mdbook serve doc`
- clear redis cache
    - `redis-cli` 进入redis
    - `flushall` 刷新所有缓存
    - `keys *` 查看所有缓存
- run jmeter
    - `jmeter -n -t [script name].jmx -l [result-file-name].jtl -e -o [report-folder-name]`
- report
    - `jmeter -g [result-file-name].jtl -o [report-folder-name]`