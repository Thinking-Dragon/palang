# Writing a prompt with custom outputs

This tutorial assumes you have already installed the Palang CLI and you are familiar with *compiling* and *running* Palang programs.

If you are not familiar with these concepts, follow the [Hello world!](/tutorials/writing-code/1-hello-world) tutorial first.

In this tutorial, you will define custom *models*. You will then use these *models* as input and output types for a prompt.

## Simple prompt to get travel ideas
Let's start with a simple example, say you have the following prompt:
```palang title="travel.palang" linenums="1"
module tutorials

prompt findTravelIdeas() -> std::Text {
    Find ten cool travel ideas for a first trip to Japan.
}
```

This prompt is very similar to the one we wrote in [Hello world!](/tutorials/writing-code/1-hello-world).

Here is an output you could get from this prompt:
```bash title="Bash command"
palang run travel.palasm --task tutorials/findtravelideas --profile groq_llama3_70b
```
``` title="Output"
Here are ten cool travel ideas for your first time in Japan: 

Visit the Fushimi Inari Shrine in Kyoto and hike the thousands of vermilion torii gates. 
Explore the Neon Districts of Tokyo, particularly Shinjuku and Shibuya. 
Relax in an Onsen, a traditional Japanese hot spring, in the Japanese Alps. 
Take a sumo tournament tour in Tokyo or Osaka. 
Try a traditional Japanese tea ceremony in Kyoto. 
Visit the Hiroshima Peace Memorial Park and Museum. 
Walk around the scenic Philosopher's Path in Kyoto. 
Take a trip to the Japanese Alps and visit the picturesque villages of Shirakawai and Gokayama. 
Experience the vibrant Osaka nightlife in Dotonbori. 
Visit the majestic Osaka Castle and its surrounding gardens.
```

This is great for a chat-bot. The response does contain the right information in *human readable language*.

What if what you wanted was a *computer readable output* (i.e. JSON). You wanted to write a website which would display these travel ideas in a list of cards on your page.

If this is what you want, the output we just got is useless because it is hard to read for a computer program.

A work-around you could use is to add instructions in your prompt to define how the output should be formatted.

## Defining your output in the prompt (bad example)
For example:
```palang title="travel.palang" linenums="1"
module tutorials

prompt findTravelIdeas() -> std::Text {
    Find ten cool travel ideas for a first trip to Japan.
    Your reponse will be a JSON array. Every idea will be an element of the array.
}
```

Here is an output you could get from this prompt:
```bash title="Bash command"
palang run travel.palasm --task tutorials/findtravelideas --profile groq_llama3_70b
```
```json title="Output"
[
    "Explore vibrant Tokyo streets and discover hidden gems in Shinjuku's Golden Gai",
    "Relax in natural hot springs in the Japanese Alps",
    "Wander through peaceful gardens and temples in Kyoto",
    "Try delicious food at Osaka's Dotonbori",
    "Take a scenic train ride to Hiroshima and visit the Peace Memorial Park",
    "Unwind on the beautiful beaches of Okinawa",
    "Visit the famous Fushimi Inari shrine in Kyoto",
    "Go hiking in the Japanese Alps and stay in a traditional ryokan",
    "Discover Japan's unique vending machine culture",
    "Take a sumo tournament tour in Tokyo"
]
```

Great! Now you have a *computer readable output*. You can `json parse` this string and iterate over the elements.

Not all is great though, look at your prompt, isn't something wrong? Your prompt now has more than one responsibility:
- Define the task to accomplish ;
- Define the response format.

What if I want to call your prompt, but this time I want the output to be `yaml`? I need to write a copy of your prompt and modify the output format instructions.

## Defining your output in a model (good example)
Introducing Palang *models*. They descibe the format of data and they can be used as *input* and *output* types for *prompts* (and later *functions*).

The `std::Text` return type we have been using in [Hello world!](/tutorials/writing-code/1-hello-world) and this in tutorial is a `model` defined in the Palang standard code repository.

Let's turn our response format instructions into a `model`:
```palang title="travel.palang" linenums="1"
module tutorials

model JsonArray {
    A JSON array.
}

prompt findTravelIdeas() -> JsonArray {
    Find ten cool travel ideas for a first trip to Japan.
}
```

Here is a possible output you can get from this program:
```bash title="Bash command"
palang run travel.palasm --task tutorials/findtravelideas --profile groq_llama3_70b
```
```json title="Output"
[
    "Explore Tokyo's neon-lit streets and try delicious food at Shinjuku's Omoide Yokocho alleyway",
    "Visit Hiroshima's Peace Memorial Park and try okonomiyaki, a local savory pancake",
    "Relax in natural hot springs at an onsen resort in the Japanese Alps",
    "Walk the famous Fushimi Inari shrine tunnel in Kyoto, adorned with thousands of vermilion torii gates",
    "Take a sushi-making class and learn the art of Japanese cuisine in Osaka",
    "Discover traditional Japanese culture at a tea ceremony in Uji, Kyoto",
    "Visit the scenic Japanese countryside and stay in a traditional ryokan inn",
    "Hike the famous Nakasendo Trail, a historic route through the Japanese mountains",
    "Explore the vibrant city of Osaka, known for its food, entertainment, and nightlife",
    "Take a day trip to Nara, famous for its ancient temples and friendly deer"
]
```

As you can see, we simplified your prompt and yet we are getting the same format as before.

## Changing the output from Json to Yaml
Now your prompt is more generic and flexible. If I want the output to be `yaml`, I can add a `yaml` model and use it as output instead.

```palang title="travel.palang" linenums="1"
module tutorials

model JsonArray {
    A JSON array.
}

model YamlArray {
    A YAML array.
}

prompt findTravelIdeas() -> YamlArray {
    Find ten cool travel ideas for a first trip to Japan.
}
```

Here is a possible output you can get from this program:
```bash title="Bash command"
palang run travel.palasm --task tutorials/findtravelideas --profile groq_llama3_70b
```
``` title="Output"
- Explore Tokyo's Neon Districts: Visit Shinjuku, Shibuya, and Asakusa to experience Japan's vibrant nightlife and colorful lights.
- Relax in an Onsen (Hot Spring): Head to the Japanese Alps or popular onsen destinations like Hakone, Kusatsu, or Beppu to unwind in natural hot springs.
- Hike the Japanese Alps: Trek through the beautiful mountains of Nagano, Niigata, or Fukushima for breathtaking scenery and tranquil villages.
- Discover Hiroshima's Peace Memorial Park: Learn about Japan's history and pay respects at the atomic bomb memorial and museum.
- Experience Kyoto's Traditional Culture: Visit the Fushimi Inari Shrine, Kinkaku-ji Temple, and Arashiyama Bamboo Grove to immerse yourself in Japan's rich heritage.
- Visit the Fushimi Inari Shrine at Sunrise: Beat the crowds and witness the serene beauty of thousands of vermilion torii gates.
- Sample Japanese Cuisine: Try popular dishes like sushi, ramen, and okonomiyaki, and explore local markets for fresh seafood and ingredients.
- Take a Bullet Train Ride: Zip across the country on the famous Shinkansen, enjoying scenic views and convenient transportation.
- Walk the Philosopher's Path in Kyoto: Stroll along the serene canal lined with cherry blossom trees and visit nearby temples and gardens.
- Visit the Miyajima Island at High Tide: Witness the famous Itsukushima Shrine appear to be floating on water.
```

Okay, awsome I can switch the output type, but when I compile the `prompt`, it only has one type, doesn't it? How does it make my prompt more flexible if I can only give it one output type?

## Prompt output type multiplexing
The answer is you can give your `prompt` more than one output types.

Let's see how:
```palang title="travel.palang" linenums="1"
module tutorials

model JsonArray {
    A JSON array.
}

model YamlArray {
    A YAML array.
}

model TravelIdeas = JsonArray | YamlArray

prompt findTravelIdeas() -> TravelIdeas {
    Find ten cool travel ideas for a first trip to Japan.
}
```

Now you can choose between `JsonArray` and `YamlArray` as outputs for your `prompt`.

Using the Palang CLI, you can specify the output type using the `--output` argument.

For example:
```bash title="Bash command"
palang run travel.palasm --task tutorials/findtravelideas --output tutorials/jsonarray --profile groq_llama3_70b
```
```json title="Output"
[
    "Explore Tokyo's Neon Districts: Visit Shinjuku, Shibuya, and Asakusa to experience Japan's vibrant nightlife and cityscape.",
    "Relax at an Onsen: Unwind in a natural hot spring surrounded by scenic landscapes.",
    "Hike the Japanese Alps: Discover picturesque villages and stunning mountain scenery in the Japanese Alps.",
    "Try a Traditional Ryokan: Stay at a traditional Japanese inn and experience the country's unique hospitality.",
    "Visit Hiroshima's Peace Memorial Park: Pay respects to the city's history and learn about its significance.",
    "Venture to Naoshima Island: Explore modern art museums, sculptures, and installations on this charming island.",
    "Take a Sushi-Making Class: Learn the art of preparing Japan's iconic dish in a hands-on cooking class.",
    "Walk the Fushimi Inari Shrine: Hike through thousands of vermilion torii gates in Kyoto.",
    "Experience a Sumo Tournament: Watch a live match of Japan's national sport at an arena in Tokyo.",
    "Visit the Zen Gardens of Kyoto: Stroll through serene gardens and temples, exemplifying Japanese tranquility."
]
```

## Next tutorial
Awesome! Now that you learned about formatting your *output*, you are ready to learn about [Writing a prompt with custom inputs](/tutorials/writing-code/3-writing-a-prompt-with-custom-inputs).
