Shi twin ts not complete come back in a couple months

<h1>Dependencies</h1>
<p>make sure you have rust and cargo downloaded.
The front end uses Serde and Tauri, but Cargo should
automatically handle downloading them.</p>

<h1>How To Run</h1>
<h2>Parsing Example</h2>
<p>Open up a terminal and navigate to the cloned repository.
after that run</p>
<code> cd solver
cargo run --example parse_example</code>
<p> The example program supports the following </p>
<ul>
<li> entering expression eg. (2+3/4) will result in
the value of the expression being printed eg. (2.75)
</li>
<li> variables can be declared by typing p<bold>n</bold>
=<bold>expression</bold> where <bold>n</bold> is an integer
</li>
<li>the program currently supports the following operators <ul>
<li>+</li>
<li>-</li>
<li>*</li>
<li>/</li>
<li>^</li>
<li>sin</li>
<li>cos</li>
<li>tan</li>
</ul>
</li>
<li> the screen can be cleared by typing clear and the program can be exited by typing exit </li>
</ul>
<h1>Frontend</h1>
make sure you sure you are in the root directory of the crate, and run
<code> cargo run</code>
currently no interactivity is available because javascript sucks and tauri is poorly documented