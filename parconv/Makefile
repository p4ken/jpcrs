all: TKY2JGD.rs touhokutaiheiyouoki2011.rs

%.rs: %.par
	cargo run $*

%.par: %.zip
	unzip -p $*.zip > $@

TKY2JGD.zip:
	curl --output $@ https://www.gsi.go.jp/common/000185226.zip

touhokutaiheiyouoki2011.zip:
	curl --output $@ https://www.gsi.go.jp/common/000184959.zip
