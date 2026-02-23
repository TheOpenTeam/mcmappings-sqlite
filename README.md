## MCM-SQLITE
用于把Minecraft的映射文件转换为SQLite数据库，供本地调用。

### 使用方法
- 先从[Release页面](https://github.com/TheOpenTeam/mcmappings-sqlite/releases)下载最新版本。对于Linux和MacOS用户请自行编译。

#### 命令(使用PowerShell演示)
在本文档中，`<>`表示可选参数,`[]`表示必要参数。
##### 创建
使用`create`命令创建一个数据库。  
`shell
.\mcmappings-sqlite create <路径>
`  
会在指定路径创建一个Database文件。若不使用路径，则在**执行目录创建mappings.db**。若文件已存在，则覆盖。  
使用`append`命令来给指定数据库新增映射。  
`shell
.\mcmappings-sqlite append <--inputs 映射列表> [--version 版本] <--db 数据库路径>
`  
映射文件可以有多个。用法：--inputs 1.txt --inputs 2.txt ...  
--version输入一个版本号。程序并不检验，只是为了区分不同版本。自动识别平台。  
若不使用--db参数，则默认以**执行目录的mappings为数据库**。
### 性能
测试版本：  
[1.21.11 Vanilla](https://piston-data.mojang.com/v1/objects/031a68bebf55d824f66d6573d8c752f0e1bf232a/client.txt)  
测试平台：  
CPU: **Intel(R) Core(TM) i7-10710U CPU @ 1.1GHz**  
RAM: **8GB DDR4**  
硬盘: **HDD**  
测试中CPU平均占用：20%-35%      
测试中内存占用：30-50MB（整个PWSH进程）  
结果：
<img width="960" height="62" alt="图片" src="https://github.com/user-attachments/assets/a457145a-fc44-4fe4-9e9d-7ba03a1ddffd" />
