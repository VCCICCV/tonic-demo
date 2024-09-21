use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from("../proto");
    // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap()); // 指定输出目录在target下
    tonic_build
        ::configure()
        .build_client(true)// 指定是否生成客户端端代码
        .out_dir("src/generated")// 指定rust文件输出目录
        .file_descriptor_set_path(out_dir.join("example_descriptor.bin")) // 生成文件描述符集
        .compile(&["../proto/example.proto"], &["../proto"])
        .unwrap();
    Ok(())
}