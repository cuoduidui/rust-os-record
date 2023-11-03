# 2023-10-23

## 1、搭建环境

[mac安装docker](https://www.runoob.com/docker/macos-docker-install.html?tdsourcetag=s_pcqq_aiomsg&wd=&eqid=ea0354ca0001ce350000000464756ccb)

[docker镜像加速](https://www.runoob.com/docker/docker-mirror-acceleration.html)

[rust镜像加速](https://www.cnblogs.com/fanqisoft/p/16905827.html)

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

1、

```shell
ERROR: failed to solve: failed to do request: Head "https://registry-1.docker.io/v2/docker/dockerfile/manifests/1": net/http: TLS handshake timeout
```

需要配置docker镜像加速。

2

```shell
could not rename component file from '/usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/sgx/abi/usercalls' to '/usr/local/rustup/tmp/dyvoupxeciqzsdtf_dir/bk': Invalid cross-device link (os error 18)
```

执行命令

```shell
rustup toolchain remove nightly
rustup toolchain install nightly
```

修改os下doker配置文件

```
env:
##增加这两行代码
	rustup toolchain remove nightly
	rustup toolchain install nightly
	
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview
```

[链接脚步说明](https://blog.csdn.net/kouxi1/article/details/126707153)

```ld
OUTPUT_ARCH(riscv)
##入口地址
ENTRY(_start)
BASE_ADDRESS = 0x80200000;
##https://blog.csdn.net/kouxi1/article/details/126707153
SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);##表示从该地址开始后面的存储进行4字节对齐
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
```

# 2023-10-24

查看联系第一章：https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter1/4first-instruction-in-kernel2.html

## 问题

执行命令报错riscv64-unknown-elf-gdb: command not found

```shell
riscv64-unknown-elf-gdb \
>     -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
>     -ex 'set arch riscv:rv64' \
>     -ex 'target remote localhost:1234'
```

# 2023-10-26

第二节课预习

## 问题

第二节课执行make run报：make[1]: *** user: No such file or directory.  Stop.

需要下载测试代码 

# 2023-11-02

## 1、[linkage = "weak"]

我们使用 Rust 的宏将其函数符号 `main` 标志为弱链接。这样在最后链接的时候，虽然在 `lib.rs` 和 `bin` 目录下的某个应用程序都有 `main` 符号，但由于 `lib.rs` 中的 `main` 符号是弱链接，链接器会使用 `bin` 目录下的应用主逻辑作为 `main` 。这里我们主要是进行某种程度上的保护，如果在 `bin` 目录下找不到任何 `main` ，那么编译也能够通过，但会在运行时报错。

```rust
#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}
```

为了支持上述这些链接操作，我们需要在 `lib.rs` 的开头加入：

```rust
#![feature(linkage)]
```

## 2、asm!

`global_asm!` 宏来嵌入全局汇编代码，而这里的 `asm!` 宏可以将汇编代码嵌入到局部的函数上下文中。相比 `global_asm!` ， `asm!` 宏可以获取上下文中的变量信息并允许嵌入的汇编代码对这些变量进行操作。由于编译器的能力不足以判定插入汇编代码这个行为的安全性，所以我们需要将其包裹在 unsafe 块中自己来对它负责。

```rust
// user/src/syscall.rs
 2use core::arch::asm;
 3fn syscall(id: usize, args: [usize; 3]) -> isize {
 4    let mut ret: isize;
 5    unsafe {
 6        asm!(
 7            "ecall",
 8            inlateout("x10") args[0] => ret,
 9            in("x11") args[1],
10            in("x12") args[2],
11            in("x17") id
12        );
13    }
14    ret
15}
```



## 3、RefCell Cell

[链接](https://www.jianshu.com/p/fa2b5a594305)

## 4、 Vectored 模式

- Direct模式：所有的中断和异常使用同一个中断入口地址，一般都会设置为这种模式。
- Vectored模式：所有异常使用同一个入口地址，但是不同的中断使用不同的入口地址。

[riscv 中断和异常处理 ](https://www.cnblogs.com/dream397/p/15687184.html)

## 5、csr riscv csrrw

https://blog.csdn.net/humphreyandkate/article/details/112941145

# 2023-11-03

