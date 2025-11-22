created: 2018-04-15
css: /css/post.css
tags: game js webgl physics

{{../templates/nav.html}}

# JavaScript Physics Engines

Under promise and over deliver! Last week I said I'd be releasing a demo of 3 physics engines I've been comparing. Well guess what?! I've got 4 to show you!

**UPDATE**: Samuel tweaked my wall demo and it's now keeping up with the big boys! I need to run a few more tests before making a final decision but I guess the key thing is to abstract it away and then it makes switching much easier. The BabylonJS plugins are great for this so that will certainly be a factor.

![Collapsing wall physics engine test](/images/engine_test.webp)

Last week I noticed that Samuel Girardin had released an alpha version of Energy.js, a partially manual JavaScript port of the Open Dynamics Engine. I hacked together a quick plugin using the examples from Samuel's [Github repo](https://github.com/samuelgirardin/Energy.js) and voilà! Energy.js is not performing quite as I'd hoped but it's early days and I've not really played around with the parameters much so take it with a pinch of salt.

So, I'm testing Cannon and Oimo which have builtin plugins for BabylonJS. Very simple. AmmoJS was a bit more tricky and doesn't have the nicest API but with a great example from JohnK over at http://www.html5gamedevs.com I was able to get it up and running. As I mentioned already, Samuel's examples provided the basis for the Energy demo.

They all behave slightly differently with this many blocks (30x30 or 900 objects) which is quite interesting. Cannon is slow but the wall seems to hang around longer. Ammo seems to have the best overall performance and doesn't drop quite as much as the others. However, I may add some different objects to get a better sense of how each engine copes with different scenarios.

I've also started playing around with Web Workers which may provide a speed boost but I may just pick a library before heading in that direction.

Finally, an interesting gotcha. Setting the impostor before setting the objects initial position caused Oimo to run out of memory. Not sure if this happens to other engines and whether it's the plugin or the engine itself but might be something to look out for.

Until next week.
