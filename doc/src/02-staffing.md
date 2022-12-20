# Staffing相关接口

## 关键接口

### Staffing创建接口

#### API

```http request
POST endpoint/staffing
Content-Type: application/json
```

```json
{
  "engageId": "152655"
}
```

#### SQL

```mysql
select eb.eng_id
from erp_budget eb
where approval_time is not null
```