GHC = ghc
OPTIONS_GHC = -dynamic --make

fuel: fuel.hs
	$(GHC) $(OPTIONS_GHC) fuel.hs

clean:
	$(RM) *.hi *.o fuel
