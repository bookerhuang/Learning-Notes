# Gin框架

概述：

 Gin 来写 API 接口。

官网：

https://gin-gonic.com/zh-cn/docs/quickstart/

# 一 Gin安装

Gin官方文档地址：https://gin-gonic.com/zh-cn/docs/

安装Gin：

```shell
go get -u github.com/gin-gonic/gin
```

# 二 简单的GET请求

## 1 将 gin 引入到代码中：

```go
import "github.com/gin-gonic/gin"
```

（可选）如果使用诸如 `http.StatusOK` 之类的常量，则需要引入 `net/http` 包：

```go
import "net/http"
```

## 2 GET请求的简单示例：

```go
package main

import "github.com/gin-gonic/gin"

func main() {
	// 创建一个服务
	ginServer := gin.Default()
	// 访问地址，处理我们的请求 Request Response
	ginServer.GET("/hello", func(context *gin.Context) {
		context.JSON(200, gin.H{"msg": "hello, world!"})
	})
	// 服务器端口
	ginServer.Run(":8082")
}
```

## 3 favicon中间件 

```shell
go get -u github.com/thinkerou/favicon
```

使用以下命令引用favicon中间件

```go
ginServer := gin.Default()
ginServer.Use(favicon.New("./皮卡丘.ico"))
```

即可改变页面页标签图标

# 三 RESTful API

RESTful API是一种API代码风格

RESTful风格的主要特点包括：

- 资源（Resources）：将应用程序的功能和数据抽象为资源，每个资源都有一个唯一的标识符（URI）。
- 统一接口（Uniform Interface）：使用统一的HTTP方法（GET、POST、PUT、DELETE等）对资源进行操作，通过不同的HTTP方法来表示不同的操作。
- 无状态（Stateless）：每个请求都是独立的，服务器不会存储客户端的状态信息，每个请求都包含足够的信息来处理请求。
- 按需响应（Response on Demand）：服务器根据客户端的请求，按需返回资源的表示形式（如JSON、XML等）。
- 超媒体驱动（HATEOAS）：通过在响应中包含超链接，使客户端能够动态地发现和访问相关资源。

以前写网站：

```go
get /user
post /create_user
post /update_user
post /delete_user
```

RESTful API风格：

```go
get /user
post /user
put /user
```

# 四 给前端响应页面

## 1 创建目录

创建目录结构如下：

![image-20230611162418826](C:\Users\Administrator\AppData\Roaming\Typora\typora-user-images\image-20230611162418826.png)

其中templates放置html文件，static结构如图所示。

## 2 编写html文件

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>我的第一个Go Web页面</title>

    <link rel = "stylesheet" href="/static/css/style.css">
    <script src = "/static/js/common.js"></script>
</head>
<body>
<h1>Some call is magic, I call it is good aim.</h1>
获取后端的数据为：
{{.msg}}
</body>
</html>
```

## 3 编写资源文件

`style.css`：

```css
body{
    background: aqua;
}
```

`common.js`：

```js
alert("1")
```

## 4 加载静态页面和资源文件

```go
package main

import (
	"github.com/gin-gonic/gin"
	"github.com/thinkerou/favicon"
	"net/http"
)

func main() {
	// 创建一个服务
	ginServer := gin.Default()
	ginServer.Use(favicon.New("./皮卡丘.ico"))

	// 加载静态页面
	ginServer.LoadHTMLGlob("templates/*")
	// 加载资源文件
	ginServer.Static("/static", "./static")	
	// 响应一个页面给前端
	ginServer.GET("/index", func(c *gin.Context) {
		// context.JSON() json数据
		c.HTML(http.StatusOK, "index.html", gin.H{
			"msg": "后端传来的数据",
		})
	})    
	// 服务器端口
	ginServer.Run(":8082")
}

```

# 五 获取请求中的参数

## 1 在地址栏获取参数

### 正常格式

```go
// url?userid=xxx&username=huangshilong 正常风格
ginServer.GET("/user/info", func(context *gin.Context) {
    userid := context.Query("userid")
    username := context.Query("username") // url中指定字段名用Query
    context.JSON(http.StatusOK, gin.H{
        "userid":   userid,
        "username": username,
    })
})
```

### RESTful格式

```go
// /user/info/1/kuangshen RESTful风格
ginServer.GET("/user/info/:userid/:username", func(context *gin.Context) {
    userid := context.Param("userid")
    username := context.Param("username") // url中未指定字段名用Param
    context.JSON(http.StatusOK, gin.H{
        "userid":   userid,
        "username": username,
    })
})
```

## 2 前端给后端传递json

```go
// 前端给后端传递json
ginServer.POST("/json", func(c *gin.Context) {
    data, _ := c.GetRawData()
    var m map[string]interface{}
    // 包装为json数据，把byte[]转化为指定的数据
    _ = json.Unmarshal(data, &m)
    c.JSON(http.StatusOK, m)

})
```

## 3 从表单中获取参数

### 增加方法：

```go
ginServer.POST("user/add", func(c *gin.Context) {
    username := c.PostForm("username")
    password := c.PostForm("password")
    c.JSON(http.StatusOK, gin.H{
        "username": username,
        "password": password,
    })
})
```

### 更改静态文件html：

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>我的第一个Go Web页面</title>

    <link rel = "stylesheet" href="/static/css/style.css">
    <script src = "/static/js/common.js"></script>
</head>
<body>
<h1>Some call is magic, I call it is good aim.</h1>
获取后端的数据为：
{{.msg}}

<form action="/user/add" method="post">
    <p>username:<input type="text" name="username"></p>
    <p>password:<input type="text" name="password"></p>
    <button type="submit">提交</button>
</form>
</body>
</html>
```

即可从表单中获取参数

# 六 路由

## 1 重定向

```go
ginServer.GET("/test", func(context *gin.Context) {
    // 重定向 301
    context.Redirect(http.StatusMovedPermanently, "http://www.baidu.com")
})
```

## 2 个性化404页面

### 创建html文件

```html
<!--templates/404.html-->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>404</title>
</head>
<body>
<h1>我的404页面</h1>
</body>
</html>
```

### 增加路由

```go
// 404 NoRoute
ginServer.NoRoute(func(context *gin.Context) {
    context.HTML(http.StatusNotFound, "404.html", nil)
})
```

## 3 路由组

```go
// 路由组
userGroup := ginServer.Group("/user")
{
    userGroup.GET("/add")
    userGroup.GET("/login")
    userGroup.GET("/logout")
}
orderGroup := ginServer.Group("order")
{
    orderGroup.GET("/add")
    orderGroup.DELETE("/delete")
}
```

# 七 中间件

## 1 定义中间件

```go
// 自定义中间件（拦截器）
func myHandler() gin.HandlerFunc {
	return func(context *gin.Context) {
		// 通过自定义的中间件，设置的值，在后续处理中只要调用了这个中间件的都可以拿到这里的参数
		context.Set("userSession", "userid-1")
		context.Next() // 放行
		// context.Abort() // 阻止
	}
}
```

## 2 添加中间件

```go
ginServer.GET("/user/info", myHandler(), func(context *gin.Context) {
    // 取出中间件的值
    usersession := context.MustGet("userSession").(string)
    log.Println("=============>", usersession)

    userid := context.Query("userid")
    username := context.Query("username")
    context.JSON(http.StatusOK, gin.H{
        "userid":   userid,
        "username": username,
    })
})
```

### 注：

如果不在函数中添加中间件，那么中间件全局有效
