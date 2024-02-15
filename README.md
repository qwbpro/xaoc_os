https://os.phil-opp.com/   全代码 把config.toml复制到.cargo目录
config.toml里面的build字段traget项改为项目x86_64-xaoc_os.json文件的绝对路劲
[build]
target = "D:\\projeckets\\rust\\xaoc_os\\x86_64-xaoc_os.json"
使用rust夜间版工具链，然后cargo run  
