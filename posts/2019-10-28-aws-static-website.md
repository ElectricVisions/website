mmd header: {{../templates/header.html}}
mmd footer: {{../templates/footer.html}}
css: /css/main.css
updated: 2020-04-29
tags: web aws cert s3


# Create a Static Website on AWS

Some notes on quickly setting up a static website with a custom apex & www domain over HTTPS/TLS using Amazon S3, Route 53, CloudFront and Let's Encrypt Certificate Authority.

## Preamble

My plan is to add some application functionality using AWS Lambda at some point
so it seemed like a useful exercise to host the static content using AWS.

I've setup 3 websites with this method. My only issues were not pointing Route 53
at S3 (and not CloudFront) to setup the certificates and being impatient while
the DNS transfer that I'd also initiated went through.

## Prerequisites

Make sure you're logged in to your AWS account **https://console.aws.amazon.com**.
If you've ever used Amazon, AWS uses the same credentials. Also, you'll want to
login to your domain registrar (I'm using (https://namecheap.com)).

It's helpful to have the following 4 tabs open to copy data from one to the
other while going through this process.

1. [https://s3.console.aws.amazon.com](https://s3.console.aws.amazon.com)
1. [https://console.aws.amazon.com/cloudfront](https://console.aws.amazon.com/cloudfront)
1. [https://console.aws.amazon.com/route53](https://console.aws.amazon.com/route53)
1. [https://ap.www.namecheap.com/](https://ap.www.namecheap.com/) (or your domain registrar)


## Register a domain

I've recently switched to https://namecheap.com. They seem to be the cheapest (
their WHOIS privacy is free), their interface is pretty simple to use and
I've not had any issues in the last few months of using them.

I used them to register this website **electricvisions.com**.

## Create S3 Buckets and upload files

### Apex domain bucket

Amazon requires the bucket name to match the domain name.
So for the apex domain our bucket will be called **electricvisions.com**.

1. Browse to **https://s3.console.aws.amazon.com**
1. Click **+ Create Bucket**
1. *Name and region*, enter **Bucket name** which should match the domain name
   e.g. `electricvisions.com`, optionally change the region and click **Next**
1. *Configure options*, click **Next**
1. *Permissions*, Uncheck **Block all public access** and click **Next**
1. *Review*, click **Create Bucket**
1. Click **Upload** and drag in your HTML files (I also have a script for this:
   `aws s3 sync output/electricvisions s3://electricvisions.com --delete`. It
   requires the aws cli tools and basically mirrors what I have locally)
1. Click the Bucket name, in our case **electricvisions.com**, to edit the settings
1. Click **Properties** tab
1. Click **Static website hosting** card
1. Click **Use this bucket to host a website**
1. Enter your root document in **Index document** (usually `index.html`)
1. Enter your error document if you have one
1. Click **Save**
1. Click **Permissions** tab
1. Click **Bucket Policy**
1. Enter the following into the editor changing `electricvisions` to your domain
   and click **Save**

```json
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Principal": "*",
            "Action": "s3:GetObject",
            "Resource": "arn:aws:s3:::electricvisions.com/*"
        }
    ]
}

```

### Www domain bucket

This will just redirect to the apex domain.

1. Click **+ Create Bucket**
1. *Name and region*, enter **Bucket name** which should match the domain name
   e.g. `electricvisions.com`, optionally change the region and click **Next**
1. *Configure options*, click **Next**
1. *Permissions*, this time you can leave the defaults and click **Next**
1. *Review*, click **Create Bucket**
1. Click the Bucket name, in our case **www.electricvisions.com**, to edit the
   settings
1. Click **Properties** tab
1. Click **Static website hosting** card
1. Click **Redirect requests**
1. Enter `electricvisions.com` for the **Target bucket or domain**
1. Click **Save**

## Configure Route 53

We need to point Route 53 at the S3 website in order to verify the TLS
certificates. Once that's done we can then point it at CloudFront.

1. Browse to **https://console.aws.amazon.com/route53**
1. Click **Hosted Zones**
1. Click **Create Hosted Zone**
1. Domain Name: **electricvisions.com**
1. Click **Create**
1. Click **Create Record Set**
1. Check *Alias* **Yes**
1. Click text box **Alias Target** and select **electricvisions.com (S3 website)**
1. Click **Create**
1. Click **Create Record Set**
1. For *Name* enter **www**
1. Check *Alias* **Yes**
1. Click text box **Alias Target** and select **www.electricvisions.com (S3 website)**
1. Click **Create**
1. Make a note of the 4 **NS** records
1. Go to **https://namecheap.com**
1. Login and click **Manage** on *electricvisions.com*
1. Under *NAMESERVERS* select **Custom DNS** from the list
1. Add the 4 NS records noted previously

## Create a CloudFront distribution

1. Browse to **https://console.aws.amazon.com/cloudfront**
1. Click **Create Distribution**
1. Under *Web* click **Get Started**
1. Click in the text box next to **Origin Domain Name** and select
   **electricvisions.com.s3.amazonaws.com** from the list
1. Click **Redirect HTTP to HTTPS** (You always want to have secure connections)
1. In **Default Root Object** enter `index.html` (or whatever you want `/` to go
   to)
1. Click **Create Distribution**

This will take between 15 to 20 minutes. You'll need to wait for it to be
deployed before you can complete the final step (`./install` ) in the
following section.

## Create TLS certificates

We'll now add TLS certificates to CloudFront using letsencrypt.org's free
service.

Diego Lapiduz (@dlapiduz) created a great [script on GitHub ](https://github.com/dlapiduz/certbot-s3front)
to automate the verification and upload of the certificates. I've added a little
script to make it a one step process for apex and www domains on Windows.

```
git clone https://github.com/PhilT/certbot-s3front
cd certbot-s3front
docker build . -t certbot-s3front
```

If you haven't yet setup `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` environment
variables you can do this once with:

```
setx AWS_ACCESS_KEY_ID <Your Amazon AWS_ACCESS_KEY_ID>
setx AWS_SECRET_ACCESS_KEY <Your Amazon AWS_SECRET_ACCESS_KEY>
```

Run the following to verify and send the certificates to CloudFront:

```
./install <your.domain.com> <your CLOUDFRONT_DESTRIBUTION_ID>
```

## Back to CloudFront

Now the certificates have been uploaded we can add our domain names

1. Browse to **https://console.aws.amazon.com/cloudfront**
1. Click on the ID of the distribution you created previously
1. Click **Edit**
1. In *Alternate Domain Names (CNAMEs)* add `electricvisions.com` and
   `www.electricvisions.com` on separate lines
1. Click **Yes, Edit**

## Back to Route 53

Now the certificates are installed we can point Route 53 at CloudFront.

1. Browse to **https://console.aws.amazon.com/route53** or click on the Route53
   tab you opened previously
1. Click **Hosted Zones**
1. Click **electricvisions.com** from the list of domains
1. Click the **electricvisions.com** **A** record
1. Click text box **Alias Target** select and delete what's there and select the
   CloudFront domain (it can take a while for the name to appear in the list, in
   that case just grab the **Domain Name** from CloudFront (something ending in
   cloudfront.net)).

