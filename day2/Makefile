GHC = ghc
OPTIONS_GHC = -dynamic --make

all: runintcode findnounverb

runintcode: runintcode.hs intcode.hs
	$(GHC) $(OPTIONS_GHC) runintcode.hs intcode.hs

findnounverb: findnounverb.hs intcode.hs
	$(GHC) $(OPTIONS_GHC) findnounverb.hs intcode.hs

clean:
	$(RM) runintcode findnounverb *.hi *.o
