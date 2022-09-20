1. 函数不支持重载
2. 注意所有权
3. 实现了 fmt::Display 以后，会隐式实现 to_string()->String，所以不需要自己再写一个to_string；
4. trait 相当于一个函数集，可以帮你定义你想要的行为；