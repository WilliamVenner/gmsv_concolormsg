fn main() {
	cxx_build::bridge("src/plugin/mod.rs")
	.include("src/plugin")
	.file("src/plugin/plugin.cpp")
	.compile("gmsv_concolormsg");
}