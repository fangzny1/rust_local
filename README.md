# RUST_LOCAL
## Rust local是使用rust和vue3写的局域网网盘工具，提供配置选项实现部分文件直接浏览器内预览的功能
**安装和使用**
- 使用`git clone`将项目克隆后使用`cargo build --release`编译对应的可执行文件，在`./target/release`下找到`rust-web`即可

## Linux下使用（启动后端）
- 使用`./rust-web`即可，第一次会自动生成配置，可看下面的配置文件信息修改


**config.toml配置**
- **share_dir** : `目前选择共享的根目录`
- **bind_addr** : `选择要绑定的IP地址，本地用127.0.0.1，局域网用0.0.0.0`
- **port** : `选择需要绑定的端口，默认0.0.0.0`
- **inline_pdf** : `bool参数，true时对pdf和图片文件使用浏览器内打开，false则直接下载`
- *使用前需要修改默认的配置，如果认为修改后有格式内容等写错不了解如何恢复，请在启动的时候加上参数r或者reset进行重置即可，注意配置文件未找到时会自动生成默认的配置文件内容*

## Linux下使用Vue3前端

- **打开./vue3目录** : `npm install`更新依赖
- **运行前端** : `npm run dev`运行Vue3

*PS:目前还在开发阶段，到时候更完善的时候会考虑弄个一键启动的程序在Releases中ww*