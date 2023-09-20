---
title: AWS organization on personal account part 3: budgets and cost reports

author: Clement Demonchy
---

Welcome to the third part in this series, this time we will setup some alerting for your AWS bills, understand how tagging done in part 2 will help get an better picture of expenses and get a proper cost dashboard.
<!--truncate-->

Budget and Budget Report
========================

Even through understanding your AWS bill itself is a complicated issue, setting a budget in order to get alert based on threshold, forecast and other metrics is relatively straight forward.

Under Billing Console you have a budget menu were you can create an budget in this screenshot we go with the simplest option, got an alert if any free tier for the whole account is reached and send a email.

Once budget are created you can ask for Budget report to be send directly by email

You will feel safer after setting some alert keep an eye on cost activity but this will not answer our question of why cost are going up, to gain some insight head to the cost explorer

Cost Explorer
=============

This a great tools provided by AWS were you can filter and agreggate your data on a lot of different parameters.

On this you can do is the ability to go back to some previously exploration by saving you current parameters using the "Save to report library" button and name it to use it again later.

There is also an option to export result as csv to feed into other system

Cost & Usage report
===================

If you want to drill deeper seeing rate, cost not limited to a service, or automate what you can do in Cost Explorer you have to create an CUR (Cost and Usage Reports).

This will upload detailed billing information in a s3 bucket to be displayed using a dashboard tool, in our example we are going to use Athena for this.

Need to wait for the first result to be export to s3, we can then find an SQL query to run on Athena in order to create the table

If you look at the end of the CUR you will find the Cost allocation Tag we set in part 2, simple tag will not be shown here contrary to Cost explorer

Here some of the previously seen examples as terraform

```
resource "aws_budgets_budget" "total_organization" {  
  name         = "total-organization-budget"  
  budget_type  = "COST"  
  limit_amount = "5" # I usually manage to stay under free tier ;)  
  limit_unit   = "USD"  
  time_unit    = "DAILY"notification {  
    comparison_operator        = "GREATER_THAN"  
    threshold                  = 100  
    threshold_type             = "PERCENTAGE"  
    notification_type          = "ACTUAL"  
    subscriber_email_addresses = ["youremail@dot.com"] 
  }  
}resource "aws_s3_bucket" "billing" {  
  bucket = "billing-eu-west-3"  
}data "aws_iam_policy_document" "billing_write_access" {  
  # READ/WRITE access for all s3 bucket   
  statement {  
    actions = [  
      "s3:GetBucketAcl",  
      "s3:GetBucketPolicy"  
    ] 
    resources = [  
      aws_s3_bucket.billing.arn,  
    ] 
    principals {  
      type        = "AWS"  
      identifiers = [<root account id>] 
    }  
  }  
  statement {  
    actions = [  
      "s3:PutObject",  
    ] 
    resources = [  
      "${aws_s3_bucket.billing.arn}/\*",  
    ] 
    principals {  
      type        = "AWS"  
      identifiers = [<root account id>] 
    }  
  }  
}
resource "aws_s3_bucket_policy" "allow_access_from_billing_account" {  
  bucket = aws_s3_bucket.billing.id  
  policy = data.aws_iam_policy_document.billing_write_access.json  
}
resource "aws_cur_report_definition" "cost_report" {  
  report_name                = "cost_report_"  
  time_unit                  = "HOURLY"  
  format                     = "Parquet"  
  compression                = "Parquet"  
  additional_schema_elements = ["RESOURCES"] 
  s3_bucket                  = aws_s3_bucket.billing.id  
  s3_prefix                  = "cost-report"  
  s3_region                  = "eu-west-3"  
  additional_artifacts       = ["ATHENA"] 
  refresh_closed_reports     = "true"  
  report_versioning          = "OVERWRITE_REPORT"  
}

```

You can use your budget alert to also check consumption of resource like RAM or Reserved Instance usage see [example](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/budgets_budget).
