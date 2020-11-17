GHC = ghc
OPTIONS_GHC = -dynamic --make

fuel: intcode.hs
	$(GHC) $(OPTIONS_GHC) runintcode.hs intcode.hs

clean:
	$(RM) runintcode *.hi *.o
