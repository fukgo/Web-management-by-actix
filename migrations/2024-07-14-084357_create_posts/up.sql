-- Your SQL goes here
-- 用户管理

create table users_table(

  uuid varchar(36) primary key,

  username varchar(25) not null,

  password varchar(25) not null,

  email varchar(255) not null,

  created_at timestamp not null DEFAULT CURRENT_TIMESTAMP,

  updated_at timestamp not null DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP

);

create table roles_table(

  id int primary key auto_increment,

  role_name varchar(25) not null unique

);

create table user_roles_table_correlation(

  user_uuid varchar(36),

  role_id int,

  primary key (user_uuid, role_id),

  foreign key (user_uuid) references users_table(uuid),

  foreign key (role_id) references roles_table(id)

);

INSERT INTO roles_table (role_name) VALUES ('admin'), ('user'), ('vip');

create table user_profile_table(

  user_uuid varchar(36) primary key,

  real_name varchar(50),

  bio text,

  avatar_url varchar(255),

  gender varchar(10),

  age int,

  foreign key (user_uuid) references users_table(uuid)

);

-- 交易信息

-- 产品类型

create table product_types_table(

  id int primary key auto_increment,

  type_name varchar(50) not null,

  type_icon varchar(255),

  created_at timestamp not null DEFAULT CURRENT_TIMESTAMP,

  updated_at timestamp not null DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP

);

create table product_table(

  id int primary key auto_increment,

  product_type_id int,

  product_name varchar(50) not null,

  product_price decimal(10,2) not null,

  -- 商品现货

  product_stock int not null,

  product_description text,

  -- 商品图标

  product_icon varchar(255),

  product_status int not null,

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