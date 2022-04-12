[![Github Workflows](https://img.shields.io/github/workflow/status/jonay2000/weblab-rs/weblab-rs?logo=github&style=for-the-badge)](https://github.com/gdemenibus/WebLabPrint/actions/workflows/ci.yml)

# WebLabPrint
A program to fetch data from the TU Delft's Weblab site, edit it on your machine and uploaded it to the desired assignment.

## Objectives (In order)
### Audience:
Students, who want to edit shit locally on their machine
TAs, who want to see things on the TA side
Teachers?? (This might be a much later requiremnt)

### Two deliverables:
One: A binary to hadle all user interaction, but it is a facade
Two: A library to handle comms with weblab.

- [ ] Communicate with weblab (Log in succesfully)
- [ ] Extract key features from Page (One fore code, one for tests, one for
  temoplate code)
	- [ ] Library code
	- [ ] Solution template 
	- [ ] Test template
	- [ ] Assignment Text
- [ ] Detect Language, else default to something else
	- Possible languages: Python, Java, Scala, Rust (Major languages used
	  by weblab)
	- Start with Rust (We can test with it without a problem)
- [ ] Upload code to weblab (Feed text file, text is now on weblab)
- [ ]  TA Side of things (Phase 2 of the development)
- [ ] Teacher side of thigns (Phase 3 of development)
- [ ] Interactive set up
- [ ] Configs you can load, with boxes to tick
	- [ ] One global, one local (per weblab excercises)

Start with: Command line interface
weblab start
weblab login
weblab init 
weblab push/pull
weblab config
weblab test 
weblab build
weblab end

