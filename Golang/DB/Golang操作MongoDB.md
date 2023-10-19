# Go语言 数据操作MongoDB

MongoDB是非关系型数据库当中功能最丰富，最像关系型数据库的。它支持的数据结构非常松散，采用类似json的bjson来存储数据，因此可以存储比较复杂的数据类型。MongoDB最大的特点就是支持的查询语言非常强大，几乎可以实现类似关系型数据库单表查询的绝大部分功能，还支持对数据建立索引

## MongoDB的一些概念

| SQL术语/概念 | MongoDB术语/概念 |              解释/说明               |
| :----------: | :--------------: | :----------------------------------: |
|   database   |     database     |                数据库                |
|    table     |    collection    |            数据库表/集合             |
|     row      |     document     |           数据记录行/文档            |
|    column    |      field       |             数据字段/域              |
|    index     |      index       |                 索引                 |
| table joins  |                  |        表连接，MongoDB不支持         |
| primary key  |   primary key    | 主键，MongoDB自动将_id字段设置为主键 |

MongoDB中指定索引插入比不指定慢很多，因为MongoDB里的每一条数据的_id值都是唯一的。

当不指定id插入数据时，其id是系统自动计算生成的。MongoDB通过计算特征值、时间、进程ID和随机数来确保生成的_id是唯一的。

而在指定id插入时，MongoDB没插一条数据，都需要检查此id可不可以用，当数据库中的数据条数太多时，这一步的查询开销会拖慢整个数据库的插入速度。

## 下载MongoDB

```
https://www.mongodb.com/download-center/community
```

```
https://www.mongodb.com/download-center/shell
```

### 创建数据库

```
use go_db;
```

### 创建集合

```
db.createCollection("student");
```

## 添加MongoDB依赖

```shell
go get go.mongodb.org/mongo-driver/mongo
```

## 连接MongoDB

### 链接数据库

```go
func Connect(ctx context.Context, opts ...*options.clientOptions)
```

connect需要两个参数，一个context和一个options.ClientOptions对象

#### 完整代码：

```go
package main

import (
	"context"
	"fmt"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"log"
)

var client *mongo.Client

func initDB() {
	// 设置客户端选项
	clientOptions := options.Client().ApplyURI("mongodb://localhost:27017")
	// 连接MongoDB
	var err error
	client, err = mongo.Connect(context.TODO(), clientOptions)
	if err != nil {
		fmt.Println("链接失败")
		log.Panic(err)
	}

	//检查连接
	err = client.Ping(context.TODO(), nil)
	if err != nil {
		log.Panic(err)
	}

	fmt.Println("链接成功")
}

func main() {
	initDB()
	defer client.Disconnect(context.TODO())
}
```

上面代码的流程就是创建链接对象option和context，然后写入mongo.Connect，Connect函数返回一个链接对象和一个错误对象，如果错误对象不为空，那就链接失败了。

可以再次测试链接：`client.Ping(context.TODO(), nil)`如果不为空，就表示链接失败

### 创建数据表的链接对象

```go
collectionStudent := client.Database("go_db").Collection("student")
```

go_db是数据库，student是数据表

### 断开链接对象

```go
client.Disconnect()
```

如果我们不再使用链接对象，那最好断开以减少资源消耗

```go
err = client.Disconnect(context.TODO())
if err != nil {
    log.Fatal(err)
}
fmt.Println("MongoDB链接已关闭。")
```

## 操作MongoDB数据库

MongoDB中的JSON文档存储在名为BSON(二进制编码的SON)的二进制表示中。与其他将JSON数据存储为简单字符串和数字的数据库不同，BSON编码扩展了ISON表示，使其包含额外的类型，如int、long、date、浮点数和decimal128。这使得应用程序更容易可靠地处理、排序和比较数据。

在go.mongodb中有两种族来使用bson数据，分别是D和RAW。

D族是使用原生Go形式来构造一个BSON对象。这个对于使用命令来操作mongoDB是十分有用的。
D()有下面4种类型:

- D：一个BSON文档，这个是有序的。
- M：一个无序的map，它除了无序之外和D是一样的（可以理解为map和bson可以转换）
- A：一个BSON形式的数组。
- E：一个D里面的单独元素。（就是文档里的一个元素）

RAW组是被用来判断是否为bytes的一个切片

你也可以用 lookup() 方法从RAW中取得一个元素，在将BSON转化为另一个形式的数据时是非常有用的（可以节省转化数据时的开销）

定义学生结构体

```go
// 定义学生结构体
type Student struct {
	Name string
	Age int
}
```

### 插入单个文档

```go
collection.InsertOne()
```

```go
// 插入单条数据
func insertData() {
    // 链接mongodb
    initDB()
    // 成功后断开mongodb
    defer client.Disconnect(context.TODO())
    
    // 初始化
    s := Student{
        Name: "hsl",
        Age: "23",
    }
    // 链接数据表对象
    collection := client.Database("go_db").Collection("student")
    // 插入单条数据
    ior, err := collection.InsertOne(context.TODO(), s)
	if err != nil {
		fmt.Println("插入失败")
	} else {
		fmt.Printf("ior.插入ID值：%v\n", ior.InsertedID)
	}
}
```

### 插入多条文档

```go
collection.InsertMany()
```

不同的是接受一个切片作为数据集合：

```go
// 插入多条数据
func insertManyData() {
    // 链接mongodb
    initDB()
    // 成功后断开mongodb
    defer client.Disconnect(context.TODO())
    // 初始化
    s := Student{
        name: "王五",
        Age: 23,
    }
    s1 := Student{
        Name: "李四",
        Age: 20,
    }
    // 声明成切片
    stus := []interface{}{s, s1}
    
    // 链接数据表对象
    collection := client.Database("go_db").Collection("student")
    // 插入多条数据
    ior, err := collection.InsertMany(context.TODO(), stus)
    if err != nil {
        log.Fatal(err)
    } else {
        fmt.Printf("ior.InsertedIDs: %v\n", ior.InsertedIDs...)
    }
}
```

### 更新单个文档

```go
collection.UpdateOne()
```

如果有多个满足条件的，只更新第一条

```go
// 修改单条数据
func updateData() {
    // 链接mongodb
    initDB()
    // 成功后断开mongodb
    defer client.Disconnect(context.TODO())
    // 链接数据表对象
    collection := client.Database("go_db").Collection("student")
    
    // filter: 包含查询操作符的文档，可以用来选择要查询的文档
    // 查询到name = 李四的文档
    filter := bson.D{{Key: "name", Value: "李四"}}
    // 修改name 为张三 &inc增加 &set设置成
    update := bson.D{
        {Key: "&set", Value: bson.D{{Key: "name", Value: "李四"}}},
    }
    ur, err := collection.UpdateOne(context.TODO(), filter, update)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("ur.ModifiedCount: %v\n", ur.ModifiedCount)
}
```

### 更新多个文档

```go
collection.UpdateMany()
```

```go
// 修改多条数据
func updateData() {
    // 链接mongodb
    initDB()
    // 成功后断开mongodb
    defer client.Disconnect(context.TODO())
    // 链接数据表对象
    collection := client.Database("go_db").Collection("student")
    
    // 查询到name = 张三的文档
    filter := bson.D{{Key: "name", Value: "李四"}}
    // 修改age增加一岁 &inc增加 &set设置成
    update := bson.D{
        {Key: "&inc", Value: bson.D{{Key: "age", Value: 1}}},
    }
    ur, err := collection.UpdateOne(context.TODO(), filter, update)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("ur.ModifiedCount: %v\n", ur.ModifiedCount)
}
```

### 删除单个文档

```go
collection.DeleteOne()
```

```go
// 删除单个文档
func deleteData() {
    // 链接mongodb
    initDB()
    // 成功后断开mongodb
    defer client.Disconnect(context.TODO())
    // 链接数据表对象
    collection := client.Database("go_db").Collection("student")
    
    // 删除name = 王五的数据
    filter := bson.D{{Key: "name", Value: "王五"}}
    dr, err := collection.DeleteOne(context.TODO(), filter)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("dr.DeletedCount: %v\n", dr.DeleteCount)
}
```

### 删除多个文档

```go
collection.DeleteMany()
```

```go
// 删除多个文档
func deleteManyData() {
    // 链接mongodb
    initDB()
    // 成功后断开mongodb
    defer client.Disconnect(context.TODO())
    // 链接数据表对象
    collection := client.Database("go_db").Collection("student")
    // 删除name=张三的数据
    filter := bson.D{{Key: "name", Value: "王五"}}
    dr, err := collection.DeleteMany(context.TODO(), filter)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("dr.DeletedCount: %v\n", dr.DeleteCount)
}
```

### 查询单个文档

```go
collection.FindOne()
```

```go
// 查找单个文档
func findData() {
    // 链接mongodb
    initDB()
    // 成功后断开mongodb
    defer client.Disconnect(context.TODO())
    // 链接数据表对象
    collection := client.Database("go_db").Collection("student")
    // 查找成功赋值给s
    var s Student
    // 查找name=zxf
    filter := bson.D{{key: "name", value: "zxf"}}
    // Decode 函数的作用是将 MongoDB 查询结果中的数据解码到 Go 语言中的结构体对象中，以便进行后续的数据处理和操作。
    err := collection.FindOne(context.TODO(), filter).Decode(&s)
    if err != nil {
        log.Fatal(err)
    } else {
        fmt.Println(s)
    }
}
```

> 查找文档需要一个filter文档，以及一个指针在里面保存结果的解码

### 查询多个文档

```go
collection.Find()
```

```go
// 查找单个文档
func findData() {
    // 链接mongodb
    initDB()
    // 成功后断开mongodb
    defer client.Disconnect(context.TODO())
    // 链接数据表对象
    collection := client.Database("go_db").Collection("student")
    // 查找成功赋值给s
    var s Student
    // 查找age=23
    filter := bson.D{{key: "age", value: "23"}}
    
    cursor, err := collection.Find(context.TODO(), filter)
    if err != nil {
        log.Fatal(err)
    }
    // 关闭上下文
    defer cursor.Close(context.TODO())
    // 定义切片
    var students []Student
    err = cursor.All(context.TODO(), &students)
    
    for _, student := range students {
        fmt.Println(student)
    }
}
```

### 复合查询

#### $regex模糊查询

```go
filter := bson.M{"name": bson.M{"$regex":"张"}}
```

```go
func mutifindData() {
	filter := bson.M{"name": bson.M{"$regex": "l"}}
	c, err := collection.Find(context.TODO(), filter)
	if err != nil {
		fmt.Println("复合查询失败")
	}

	var stus []Student
	err = c.All(context.TODO(), &stus)
	if err != nil {
		fmt.Println("赋值失败")
	}

	defer c.Close(context.TODO()) // 关闭上下文

	for _, student := range stus {
		fmt.Println(student)
	}
}
```



#### in($in)包含和no in($nin)不包含

```go
filter := bson.M{"name": bson.M{"$in":[]string{"张三","李四"}}}
```

```go
func mutifindData() {
	filter := bson.M{"name": bson.M{"$in": []string{"hsl1", "hsl2", "hsl"}}}
	c, err := collection.Find(context.TODO(), filter)
	if err != nil {
		fmt.Println("复合查询失败")
	}

	var stus []Student
	err = c.All(context.TODO(), &stus)
	if err != nil {
		fmt.Println("赋值失败")
	}

	defer c.Close(context.TODO()) // 关闭上下文

	for _, student := range stus {
		fmt.Println(student)
	}
}

```



#### and($and)和

```go
filter := bson.M{"$and": []bson.M{{"name": "张三"}, {"age": 18}}}
```

```go
func mutifindData() {
	filter := bson.M{"$and": []bson.M{{"name": "hsl1"}, {"age": 46}}}
	c, err := collection.Find(context.TODO(), filter)
	if err != nil {
		fmt.Println("复合查询失败")
	}

	var stus []Student
	err = c.All(context.TODO(), &stus)
	if err != nil {
		fmt.Println("赋值失败")
	}

	defer c.Close(context.TODO()) // 关闭上下文

	for _, student := range stus {
		fmt.Println(student)
	}
}
```



#### or($or)或

```go
filter := bson.M{"$or": []bson.M{{"name": "张三"}, {"age": 20}}}
```

```go
func mutifindData() {
	filter := bson.M{"$or": []bson.M{{"name": "hsl1"}, {"age": 25}}}
	c, err := collection.Find(context.TODO(), filter)
	if err != nil {
		fmt.Println("复合查询失败")
	}

	var stus []Student
	err = c.All(context.TODO(), &stus)
	if err != nil {
		fmt.Println("赋值失败")
	}

	defer c.Close(context.TODO()) // 关闭上下文

	for _, student := range stus {
		fmt.Println(student)
	}
}
```



#### 比较函数

- `!=`（$ne）
- `>` （$gt）
- `<`（$lt)
- `>=`（$gte）
- `<=`（$lte）

```go
filter := bson.M{"age": bson.M{"$gt": 18}}
```

```go
func mutifindData() {
	filter := bson.M{"age": bson.M{"$gt": 25}}
	c, err := collection.Find(context.TODO(), filter)
	if err != nil {
		fmt.Println("复合查询失败")
	}

	var stus []Student
	err = c.All(context.TODO(), &stus)
	if err != nil {
		fmt.Println("赋值失败")
	}

	defer c.Close(context.TODO()) // 关闭上下文

	for _, student := range stus {
		fmt.Println(student)
	}
}
```



#### 聚类聚合函数

- $sum 计算总和
- $avg 计算平均值
- $min 获取集合中所有文档对应值的最小值
- $max 获取集合中所有文档对应值的最小值
- $first 根据资源文档的排序获取第一个文档数据
- $last 根据资源文档的排序获取最后一个文档数据
- $push 在结果文档中插入值到一个数据中
- $addToSet 在结果文档中插入值到一个数组中，但不创建副本

定义最大时间

```go
opts := options.Aggregate().SetMaxTime(2 * time.Second)
```

定义查询语句

```go
groupStage := bson.D{{key: "$group", value: bson.D{{Key: "_id", Value: "$major"}, {Key: "ageAvg", value: bson.D{kET: "$avg", Value: "$age"}}}}}
```

查询

```go
result, err := collection.Aggregate(context.TODO(), mogo.Pipeline{groupStage}, opts)
```

赋值，注意这里类型可以自己定义，也可以直接用bson.M

```go
var students []bson.M
err = result.All(context.TODO(), &students)
```

```go
// 复合查询
func findGroupData() {
	// 定义最大时间
	opts := options.Aggregate().SetMaxTime(2 * time.Second)

	// 查询语句 age和
	// groupStage := bson.D{{"$group", bson.D{{"_id", "$major"}, {"ageSum", bson.D{{"$sum", "$age"}}}}}}

	// 查询语句 age平均值
	// groupStage := bson.D{{"$group", bson.D{{"_id", "$major"}, {"ageAvg", bson.D{{"$avg", "$age"}}}}}}

	// 查询语句 age最小值
	// groupStage := bson.D{{"$group", bson.D{{"_id", "$major"}, {"agemin", bson.D{{"$min", "$age"}}}}}}

	// 查询语句 age最大值
	groupStage := bson.D{{"$group", bson.D{{"_id", "$major"}, {"ageMax", bson.D{{"$max", "$age"}}}}}}

	// 查询
	result, err := collection.Aggregate(context.TODO(), mongo.Pipeline{groupStage}, opts)
	if err != nil {
		fmt.Println("查询失败")
	}

	// 关闭上下文
	defer result.Close(context.TODO())

	// bson.M
	var students []bson.M
	err = result.All(context.TODO(), &students)

	// 自定义切片
	// var students []Student
	// err = result.All(context.TODO(), &students)

	for _, student := range students {
		fmt.Println(student)
	}
}
```

