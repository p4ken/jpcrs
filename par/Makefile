.PHONY: all
all: TKY2JGD.in touhokutaiheiyouoki2011.in

%.in: %.par
	cargo run --package conv < $*.par > $*.in

%.par: %.zip
	unzip -p $*.zip > $@

TKY2JGD.zip:
	curl --output $@ https://www.gsi.go.jp/common/000185226.zip

touhokutaiheiyouoki2011.zip:
	curl --output $@ https://www.gsi.go.jp/common/000184959.zip
