updated: 2020-11-12
tags: deploy aws


# Automatic Let's Encrypt TLS Certificates

Here's how I renew the TLS Certificates on my 3 websites every 3 months with one command.

## Prerequisites

This post assumes you have already setup your static website with Let's Encrypt
Certificates, using my [previous article](2019-10-28-aws-static-website.html).

## Too late?

If the certificates have already expired, make sure you switch **Redirect HTTP to
HTTPS** to **HTTP and HTTPS** in Cloudfront otherwise the certificate
installation will fail as certbot needs to be able to browse to the site.

To do this:

1. Browse to **https://console.aws.amazon.com/cloudfront**
1. Click on the **ID** of the expired distribution
1. Click the **Behaviours** tab
1. Select the first line in the table and click **Edit**
1. Change **Viewer Protocol Policy** to **HTTP and HTTPS**
1. Scroll down and click **Yes, Edit** at the bottom right

## Create a script

I have 3 sites that all need renewing at the same time. I'll use certbot which
is a Docker image from [my repo](https://github.com/PhilT/certbot-s3front.git)
that we setup in the [previous article](2019-10-28-aws-static-website.html).

First, make sure your `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` are set in your
environment. This can be done from the command-line with:

```
setx AWS_ACCESS_KEY_ID <your id>
setx AWS_SECRET_ACCESS_KEY <your key>
```

Once you've done that you'll never need to do that again unless you reinstall your
machine.

Then in the cloned [certbot](https://github.com/PhilT/certbot-s3front.git)
folder make an `install-all.ps1` file.

Then for each website you need a line:

```
./install <your.domain.com> <your distribution id>
```

Save the file and run with `./install-all`.

The certificates should install automatically.

Don't forget, if the certificates had already expired, make sure you switch
back to **Redirect HTTP to HTTPS** in Cloudfront. Similar to the steps in the
first part of this article:

1. Browse to **https://console.aws.amazon.com/cloudfront**
1. Click on the **ID** of the expired distribution
1. Click the **Behaviours** tab
1. Select the first line in the table and click **Edit**
1. Change **Viewer Protocol Policy** to **Redirect HTTP to HTTPS**
1. Scroll down and click **Yes, Edit** at the bottom right

## Conclusion

As Let's Encrypt is a free service its certificates are only valid for 3 months.
I wanted a single command that I could use to renew all my certificates easily.
This is it - providing I remember to renew them before they expire!
