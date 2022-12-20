# Budget相关

## 相关流程

### Budget创建

1. 模糊搜索code
2. 预创建code
3. 创建code
4. 进入budget页面后获取字段表数据（bu，ou，mu，smu，节假日和职级）
5. 进入budget页面后获取detail，codeList，totalHoursFee和log信息
6. 进入budget页面后获取系统配置，chargeReta，WorkHoursRatio，GdsChargeRateMinAndMax

### Budget Booking创建

## 关键接口

### Budget创建接口

#### API

```http request
POST endpoint/erp/budget/add
Content-Type: application/json
```

```json
{
  "perpo": "261939",
  "financeYear": "2023",
  "engageCode": "23299143",
  "mainServiceLine": 2,
  "subServiceLine": 1,
  "engNature": 1,
  "sectorNature": 1
}
```

#### 需要准备数据 budget_create_arg.csv

```mysql
select substr(be.name,1,7) as name_query,
       be.`code`,
       sg.main_service_line,
       sg.sub_service_line,
       if(be.py_engage_code != '', 0, 1) as engage_nature
from basic_engage be
         left join syscon_area sa on be.area_id = sa.id
         left join syscon_group sg on sa.group_id = sg.id
where be.data_status = 1
  and be.deleted = 0
  and sg.main_service_line is not null
  and sg.sub_service_line is not null;
```

### 添加budget Booking

#### API

```http request
POST endpoint/eic_booking/create
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
    "id": "",
    "staffId": "526",
    "startTime": 1666573200000,
    "endTime": 1666951200000,
    "totalHours": 40
  }
]
```

#### 需要准备数据 budget_create_booking_arg.csv

```mysql
select eb.eng_id
from erp_budget eb
where submit_time is null
```