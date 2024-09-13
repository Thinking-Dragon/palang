# Hello world!

## Installing the command line interface (CLI):
First, you need to install the Palang CLI.

```bash title="Ubuntu"
snap install palang
```

```bash title="Debian"
apt install palang
```

## Writing your first Palang program
The simplest way to write a Palang program is to create a `.palang` file anywhere on your computer.

Let's create `hello_world.palang`.

The Palang programming language has three fundamental constructs: `model`, `prompt` and `function`.

- *Models* describe the format of data which is given as input and obtained as output of *prompts* and *functions*.
- *Prompts* describe in human language a task to accomplish. The task will be accomplished using a large language model.
- *Functions* are normal programming language functions. You can define variables, use operators and invoke other *functions* and *prompts*.

For this example, we will define a prompt called `greet`, which will ask the LLM to say "Hello world!".

The prompt will not accept any input and will return a string of text (`std::Text`).

```palang title="hello-world.palang" linenums="1"
module tutorials

prompt greet() -> std::Text {
    Say "Hello world!" and nothing else.
}
```

## Compiling your first Palang program
To run your program, you need to first compile it into an assembly language called `palasm`, this is the language that the Palang virtual machine understands.

To compile your program, run:
```bash
palang compile --source ./hello_world.palang --target ./hello_world.palasm
```

If you did everything correctly, you now have a `hello_world.palasm` file in your working directory.

## Creating a profile to tell Palang which LLM to use
Before you can run your program, we need to talk briefly about profiles.

Profiles are `.yaml` files that tell the Palang virtual machine which `LLM service` to use (OpenAI, Groq, Ollama, etc.), which `model` to use in the service and other metadata like the `temperature` and `max_tokens`.

In this tutorial, we are going to use Groq. Not only is Groq very fast thank to their specialized hardware, but at the time of writing this tutorial Groq also has a generous free-tier. That makes it a great service for a beginner.

The profile `.yaml` files are usually stored in `~/.local/share/palang/profiles` for usage by the local user only and in `/usr/share/palang/profiles` for system wide usage by all users.

You can also specify you own directory for profiles using the `--profiles-directory` argument when running `palang run [...]`.

For this tutorial, we will create our profile for Groq under `~/.local/share/palang/profiles/groq_llama3_70b.yaml`.

Here is the contents of the profile:
```yaml title="groq_llama3_70b.yaml" linenums="1"
llm: groq
model: llama3-70b-8192
temperature: 1
max_tokens: 1024
```

## Running your first Palang program
You now have everything you need to run your first Palang program!

In the terminal, run:
```bash
palang run ./hello_world.palasm --task tutorials/greet --profile groq_llama3_70b
```

You should get the following response:
```bash
Hello world!
```

## Next tutorial
Now that you have written and ran your first Palang program, you are ready to learn about [Writing a prompt with custom inputs and outputs](/palang/tutorials/writing-code/2-writing-a-prompt-with-custom-outputs).
