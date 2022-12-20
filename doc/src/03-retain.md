# Retain相关接口

## 关键接口

### Retain Query

#### API

```http request

```

```mysql
### 所有EIC的ID，GPN，BU
select bs.id as eic_id, bs.gpn as eic_gpn, bs.bu_code
from basic_staff bs
where bs.data_status = 1
  and bs.deleted = 0
  and bs.service_line_code = '01'
  and bs.gpn not like 'XE%'
  and bs.rank_code in ('214', '213', '212', '211', '323', '322', '321');
### 所有Staff的ID，GPN，BU
select bs.id as staff_id,bs.gpn as staff_gpn, bs.bu_code
from basic_staff bs
where bs.data_status = 1
  and bs.deleted = 0
  and bs.service_line_code = '01'
  and bs.gpn not like 'XE%'
  and rank_code in ('423', '422', '421', '442', '441', '51', '511');
```

### Retain创建接口

#### API

```http request
POST endpoint/booking/create
Content-Type: application/json
```

```json
[
  {
    "ghost": false,
    "engagementCodeId": "152655",
    "engagementType": "1",
    "bookingType": "101",
    "extension": "{\"comment\":\"\",\"location\":\"\"}",
    "id": "",
    "staffId": "66454",
    "startTime": 1668992400000,
    "endTime": 1669024800000,
    "totalHours": 8
  }
]
```

#### SQL

```mysql
-- 选择所有可以下retain booking的engage code id和type
select id, eng_type
from basic_engage
where data_status = 1
  and deleted = 0;
-- 选择所有在职staff或者zz need的id
select id as staff_id
from basic_staff
where data_status = 1
  and deleted = 0
  and (user_type = 0 or user_type = 2);
```

### Retain更新接口

#### API

```http request
POST endpoint/booking/update
Content-Type: application/json
```

```json
[
  {
    "ghost": false,
    "engagementCodeId": "152655",
    "engagementType": "1",
    "bookingType": "111",
    "extension": "{\"comment\":\"\",\"location\":\"\"}",
    "id": "63565badbcc11f77ca4affde",
    "staffId": "66454",
    "startTime": 1669078800000,
    "endTime": 1669111200000,
    "totalHours": 8
  }
]
```

#### SQL

### Retain查询接口

#### API

```http request
POST endpoint/basic/staff/listByViewConfig
Content-Type: application/json
```

```json
{
  "start": 0,
  "size": 2000,
  "viewConditionQOS": [
    {
      "condition": "=",
      "fieldCode": "CN413",
      "fieldName": "bu_code",
      "operation": "and"
    },
    {
      "condition": "=",
      "fieldCode": "1",
      "fieldName": "data_status",
      "operation": "and"
    }
  ]
}
```

```http request
POST endpoint/booking/listByViewConfig
Content-Type: application/json
```

```json
{
  "staffIds": "11076,12447,...(avg: 60)",
  "startDate": 1663722030983,
  "endDate": 1675818030983,
  "viewConditionQOS": []
}
```

```http request
GET basic/engage/listByIds?ids=152476,144046
```

#### SQL

```mysql
-- 查看各个区域resource数量
select bu_code, count(id)
from basic_staff
where data_status = 1
  and deleted = 0
group by bu_code
order by count(id) desc;
-- CN403,3033
-- CN203,2822
-- CN401,2647
-- CN201,1990
-- HK001,1308
-- 筛选某个区域的所有resource id
drop table if exists temp_all_resource_group_concat;
create table temp_all_resource_group_concat
(
    resource_id_list text
);

drop procedure if exists generate_random_resource_group_concat_list;

create procedure generate_random_resource_group_concat_list(bu_code varchar(64))
begin

    declare v_max int unsigned default 100;
    declare v_counter int unsigned default 0;

    start transaction;
    while v_counter < v_max
        do
            insert into temp_all_resource_group_concat (resource_id_list)
            select group_concat(id) as resource_id_list
            from (select id
                  from basic_staff
                  where data_status = 1
                    and deleted = 0
                    and bu_code = bu_code
                  order by RAND()
                  limit 60) sub;
            set v_counter = v_counter + 1;
        end while;
    commit;
end;

call generate_random_resource_group_concat_list('CN403');
call generate_random_resource_group_concat_list('CN203');
call generate_random_resource_group_concat_list('CN401');
call generate_random_resource_group_concat_list('CN201');
call generate_random_resource_group_concat_list('HK001');

select *
from temp_all_resource_group_concat
order by resource_id_list;
-- 筛选某个区域的所有engagement id
drop table if exists temp_all_engagement_group_concat;
create table temp_all_engagement_group_concat
(
    engagement_id_list text
);

drop procedure if exists generate_random_engagement_group_concat_list;

create procedure generate_random_engagement_group_concat_list(bu_code varchar(64))
begin

    declare v_max int unsigned default 20;
    declare v_counter int unsigned default 0;

    start transaction;
    while v_counter < v_max
        do
            insert into temp_all_engagement_group_concat (engagement_id_list)
            select group_concat(id) as engagement_id_list
            from (select id
                  from basic_engage
                  where data_status = 1
                    and deleted = 0
                    and bu_code = bu_code
                  order by RAND()
                  limit 60) sub;
            set v_counter = v_counter + 1;
        end while;
    commit;
end;

call generate_random_engagement_group_concat_list('CN403');
call generate_random_engagement_group_concat_list('CN203');
call generate_random_engagement_group_concat_list('CN401');
call generate_random_engagement_group_concat_list('CN201');
call generate_random_engagement_group_concat_list('HK001');

select *
from temp_all_engagement_group_concat
order by engagement_id_list;
```