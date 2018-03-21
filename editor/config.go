package editor

import (
	"fmt"
	"path/filepath"

	"github.com/BurntSushi/toml"
	homedir "github.com/mitchellh/go-homedir"
)

// Config is
type Config struct {
	Modal bool
}

func loadConfig() *Config {
	c := &Config{
		Modal: false,
	}
	home, err := homedir.Dir()
	if err != nil {
		return c
	}
	path := filepath.Join(home, ".crane", "config.toml")
	_, err = toml.DecodeFile(path, c)
	if err != nil {
		fmt.Println("load config error", err)
	}
	return c
}