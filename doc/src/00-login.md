# Login相关接口

## 关键接口

### 登录接口

#### API

```http request
POST endpoint/admin_user/login
Content-Type: multipart/form-data
```

```json
{
  "username": "CN011002428",
  "password": "123456"
}
```

#### 需要准备数据 login_staff_gpn_list.csv

```mysql
## 所有EIC
select id as staff_id, gpn as staff_gpn
from basic_staff
where data_status = 1
  and service_line_code = '01'
  and gpn not like 'XE%'
  and rank_code in ('423','422','421','442','441','51','511');
## 所有EIC
select id as eic_id, gpn as eic_gpn
from basic_staff
where data_status = 1
  and service_line_code = '01'
  and gpn not like 'XE%'
  and rank_code in ('214', '213', '212', '211', '323', '322', '321');
```

