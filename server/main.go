package main

import (
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/log"
	"github.com/gofiber/fiber/v2/middleware/logger"
	// // "github.com/gofiber/fiber/v2/utils"
)

func main() {
	DrawStart()
	
	env := ParseEnvConfig()
	log.SetLevel(env.LogLevel)
	log.Debugf("Set Log Level to: %s\n", env.LogLevel)
	
	// Create the app
	app := fiber.New()
	app.Use(logger.New(logger.Config{
		DisableColors: !env.LogColor,
	}))
	
	DrawSection("Asset Paths")
	
	DrawLine("Manual", env.ManualPath)
	app.Get("/manual", func(ctx *fiber.Ctx) error {
		return ctx.Redirect("/manual/index.html")
	})
	app.Static("/manual", env.ManualPath, fiber.Static{
		Compress: true,
		MaxAge:   3600,
	})
	
	DrawLine("Legacy", env.LegacyPath)
	app.Static("/legacy", env.LegacyPath, fiber.Static{
		Compress: true,
		MaxAge:   3600,
	})
	
	DrawEnd()

	addr := env.Address
	var err error
	if addr.IsHttps() {
		err = app.ListenTLS(addr.Port, addr.CertPem, addr.CertKey)
	} else {
		err = app.Listen(addr.Port)
	}
	log.Fatal(err)
}
