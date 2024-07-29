
-- 用户管理
create table user_status_table (
  id int primary key auto_increment,
  status_name varchar(20) not null unique,
  status_description varchar(50) not null
);
INSERT INTO user_status_table (status_name,status_description) VALUES ('active','账号正常'), ('inactive','账号异常'), ('banned','账号禁用');

create table users_table(

  uuid varchar(36) primary key unique,

  username varchar(25) not null unique,

  password varchar(25) not null,

  email varchar(255) not null unique,

  user_status_id int not null default 1,

  created_at timestamp not null DEFAULT CURRENT_TIMESTAMP,

  updated_at timestamp not null DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

  foreign key (user_status_id) references user_status_table(id)

);

create table roles_table(

  id int primary key auto_increment,

  role_name varchar(25) not null unique,

  role_description varchar(50) not null

);

create table user_roles_table_correlation(

  user_uuid varchar(36),

  role_id int,

  --  role_description varchar(50),

  primary key (user_uuid, role_id),

  foreign key (user_uuid) references users_table(uuid),

  foreign key (role_id) references roles_table(id)

);

INSERT INTO roles_table (role_name,role_description) VALUES ('admin','管理员'), ('vip','超级用户'), ('ordinary','普通用户');

create table user_profile_table(

  user_uuid varchar(36) primary key,

  real_name varchar(50),

  bio varchar(255),

  avatar_url varchar(255),

  gender enum('male','female') not null,

  age int,

  check (age>=18 and age <= 80),

  foreign key (user_uuid) references users_table(uuid)

);

-- 交易信息

-- 产品类型

create table product_types_table(

  id int primary key auto_increment,

  type_name varchar(50) not null,

  type_icon varchar(255),

  description varchar(255) not null,

  created_at timestamp not null DEFAULT CURRENT_TIMESTAMP,

  updated_at timestamp not null DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP

);

INSERT INTO product_types_table (type_name, description) VALUES
('food', '食品类商品'),
('clothes', '服装类商品'),
('electronics', '电子产品'),
('daily necessities', '日用品');

create table product_status_table (
  id int primary key auto_increment,
  status_name varchar(20) not null unique,
  status_description varchar(20) not null
);
INSERT INTO product_status_table (status_name, status_description) VALUES
('in_stock', '在售'),
('out_of_stock', '售完'),
('discontinued', '停产');


create table product_table(

  id int primary key auto_increment,

  product_type_id int not null,

  product_name varchar(50) not null,

  product_price decimal(10,2) not null,

  -- 商品现货

  product_stock int not null,

  product_description varchar(255),

  -- 商品图标

  product_icon varchar(255),

  product_status_id int not null,

  created_at timestamp not null DEFAULT CURRENT_TIMESTAMP,

  updated_at timestamp not null DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

  foreign key (product_type_id) references product_types_table(id)

);

-- 支付方式表

create table payment_methods_table(

  id int primary key auto_increment,

  payment_method_name varchar(50) not null,

  payment_method_icon varchar(255)

);

INSERT INTO payment_methods_table (payment_method_name) VALUES ('微信支付') , ('支付宝支付'), ('银行卡支付');

-- 订单信息表

create table order_table(

  id int primary key auto_increment,

  user_uuid varchar(36) not null,

  -- 只考虑数量

  created_at timestamp not null DEFAULT CURRENT_TIMESTAMP,

  updated_at timestamp not null DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

  foreign key (user_uuid) references users_table(uuid)

);
-- 订单状态表
create table order_status_table (
  id int primary key auto_increment,
  status_name varchar(50) not null unique,
  status_description varchar(50) not null
);
-- 待付款、已付款、已发货、已完成等
INSERT INTO order_status_table (status_name, status_description) VALUES
('pending_payment', '待付款'),
('paid', '已付款'),
('shipped', '已发货'),
('completed', '已完成'),
('cancelled', '已取消'),
('refunded', '已退款');





-- 订单详情表

create table order_detail_table(

  id int primary key auto_increment,

  order_id int not null,

  product_id int not null,

  product_quantity int not null,

  -- 订单金额

  unit_price decimal(10,2) not null,

  created_at timestamp not null DEFAULT CURRENT_TIMESTAMP,

  updated_at timestamp not null DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

  order_status_id int not null,

  foreign key (order_status_id) references order_status_table(id),

  foreign key (order_id) references order_table(id),

  foreign key (product_id) references product_table(id)

);

-- 支付信息表

create table payment_table(

  id int primary key auto_increment,

  order_id int not null,

  payment_amount decimal(10,2) not null,

  payment_status int not null,

  payment_method_id int not null,

  created_at timestamp not null DEFAULT CURRENT_TIMESTAMP,

  updated_at timestamp not null DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

  foreign key (order_id) references order_table(id),

  foreign key (payment_method_id) references payment_methods_table(id)

);