# 2023-10-23

## 1、搭建环境

[mac安装docker](https://www.runoob.com/docker/macos-docker-install.html?tdsourcetag=s_pcqq_aiomsg&wd=&eqid=ea0354ca0001ce350000000464756ccb)

[docker镜像加速](https://www.runoob.com/docker/docker-mirror-acceleration.html)

安装成功后切换分支 执行命令：

```shell
git clone https://github.com/rcore-os/rCore-Tutorial-v3.git
$ cd rCore-Tutorial-v3
$ make build
$ cd os
$ make run
##备注 原代码中 rustc以来版本的是1.6.4 但是cargo-binutils依赖版本为最低1.6.5 只需要修改rust工具链版本就可以 rust-##toolchain.toml中channel 修改为"nightly-2023-10-21" (拉错代码了 秋季代码中无需修改)

```

在docker环境中运行项目 项目启动成功

```shell
[rustsbi] RustSBI version 0.3.0-alpha.4, adapting to RISC-V SBI v1.0.0
.______       __    __      _______.___________.  _______..______   __
|   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
|  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
|      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
|  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
| _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|
[rustsbi] Implementation     : RustSBI-QEMU Version 0.2.0-alpha.2
[rustsbi] Platform Name      : riscv-virtio,qemu
[rustsbi] Platform SMP       : 1
[rustsbi] Platform Memory    : 0x80000000..0x88000000
[rustsbi] Boot HART          : 0
[rustsbi] Device Tree Region : 0x87000000..0x87000ef2
[rustsbi] Firmware Address   : 0x80000000
[rustsbi] Supervisor Address : 0x80200000
[rustsbi] pmp01: 0x00000000..0x80000000 (-wr)
[rustsbi] pmp02: 0x80000000..0x80200000 (---)
[rustsbi] pmp03: 0x80200000..0x88000000 (xwr)
[rustsbi] pmp04: 0x88000000..0x00000000 (-wr)
[kernel] Hello, world!

```

### 问题：



```shell
ERROR: failed to solve: failed to do request: Head "https://registry-1.docker.io/v2/docker/dockerfile/manifests/1": net/http: TLS handshake timeout
```

需要配置docker镜像加速。

```shell
could not rename component file from '/usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/sgx/abi/usercalls' to '/usr/local/rustup/tmp/dyvoupxeciqzsdtf_dir/bk': Invalid cross-device link (os error 18)
```

执行命令

```shell
rustup toolchain remove nightly
rustup toolchain install nightly
```

