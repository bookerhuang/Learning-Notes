package util

import (
	"testing"
)

var commTestData []commStruct

type commStruct struct {
	Group         string
	Sizestr       string
	ExpectSize    int64
	ExpectSizestr string
}

func initCommonData() {
	commTestData = []commStruct{
		{"B", "1b", B, "1B"},
		{"B", "100b", 100 * B, "100B"},
		{"KB", "1kb", KB, "1KB"},
		{"KB", "100kb", 100 * KB, "100KB"},
		{"MB", "1mb", MB, "1MB"},
		{"MB", "1000mb", 1000 * MB, "1000MB"},
		{"TB", "1tb", TB, "1TB"},
		{"PB", "1pb", PB, "1PB"},
		{"unknown", "1G", 100 * MB, "100MB"},
	}
}

// 可以在这里进行一些初始化操作，比如数据库连接，读取配置文件等
func TestMain(m *testing.M) {
	initCommonData()
	m.Run()
}

// 功能测试
func TestParseSize(t *testing.T) {
	t.Parallel() // 并行测试
	if testing.Short() {
		t.Skip("short mod 跳过了测试")
	}
	testData := commTestData
	for _, data := range testData {
		size, sizestr := ParseSize(data.Sizestr)
		if size != data.ExpectSize || sizestr != data.ExpectSizestr {
			t.Errorf("测试结果不符合预期：%+v\n", data)
		}
	}
}

func BenchmarkParseSize(b *testing.B) {
	for i := 0; i < b.N; i++ {
		ParseSize("100mb")
	}
}

func BenchmarkParseSize2(b *testing.B) {
	b.RunParallel(func(pb *testing.PB) {
		for pb.Next() {
			ParseSize("10gb")
		}
	})
}

func FuzzPraseSize(f *testing.F) {
	f.Fuzz(func(t *testing.T, a string) {
		size, sizeStr := ParseSize(a)
		if size == 0 || sizeStr == "" {
			t.Error("测试结果不符合预期")
		}
	})
}
