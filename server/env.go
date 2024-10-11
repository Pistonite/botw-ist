package main
// Environment variables

import (
    "fmt"
	"strings"
    "path"
    "strconv"

	"github.com/gofiber/fiber/v2/log"

	"github.com/plaid/go-envvar/envvar"
)

// Raw values from environment variables
type RawEnv struct {
	LogLevel string `envvar:"ISS_LOG" default:"info"`
    LogColor bool `envvar:"ISS_LOG_COLOR" default:"true"`
    Address string `envvar:"ISS_ADDRESS"`
    Cert string `envvar:"ISS_CERT" default:""`
    ManualPath string `envvar:"ISS_MANUAL_PATH"`
    LegacyPath string `envvar:"ISS_LEGACY_PATH"`
}

type EnvConfig struct {
	// Logging Level
    LogLevel log.Level
    // Log with color
    LogColor bool
    // Path to the manual app
    ManualPath string
    // Path to the legacy app
    LegacyPath string
    // Addresses
    Address EnvAddress
}

type EnvAddress struct {
    // The string for the port to listen on (e.g. ":80")
    Port string
    // The domain part of the host URL (e.g. "example.domain.org")
    Domain string
    // The orogin part of the host URL (e.g. "http://example.domain.org")
    Origin string
    // Redirect Url for the app (e.g. "http://example.domain.org/api/auth/cb")
    RedirectUrl string

    // Path to cert.pem file. Required if https
    CertPem string
    // Path to cert.key file. Required if https
    CertKey string
}

func ParseEnvConfig() *EnvConfig {
    raw := RawEnv{}
	if err := envvar.Parse(&raw); err != nil {
		log.Fatal(err)
	}

    address, err := raw.ParseAddress();
    if err != nil {
        log.Fatal(err)
    }

    return &EnvConfig {
        LogLevel: raw.ParseLogLevel(),
        LogColor: raw.LogColor,
        ManualPath: raw.ManualPath,
        LegacyPath: raw.LegacyPath,
        Address: *address,
    }
}

func (this *RawEnv) ParseLogLevel() log.Level {
	switch strings.ToLower(this.LogLevel) {
	case "debug":
		return log.LevelDebug
	case "info":
		return log.LevelInfo
	case "warn":
		return log.LevelWarn
	default:
		return log.LevelError
	}
}

func (this *RawEnv) ParseAddress() (*EnvAddress, error) {
    addrs := EnvAddress{}

    hostParts := strings.Split(this.Address, ":")
    hostPartLength := len(hostParts)
    if hostPartLength < 2 {
        return nil, fmt.Errorf("bad host format: %s", this.Address)
    }
    rawPort := hostParts[hostPartLength-1]
    if _, err := strconv.ParseUint(rawPort, 0, 16); err != nil {
        return nil, fmt.Errorf("invalid port: %s", rawPort)
    }

    domainWithScheme := strings.Join(hostParts[:hostPartLength-1], ":")
    addrs.Port = ":" + rawPort

    redirectUrlBuilder := strings.Builder{}
    redirectUrlBuilder.WriteString(domainWithScheme)
    if rawPort != "80" && rawPort != "443" {
        redirectUrlBuilder.WriteByte(':')
        redirectUrlBuilder.WriteString(rawPort)
    }
    addrs.Origin = redirectUrlBuilder.String()
    if strings.HasPrefix(domainWithScheme, "http://") {
        addrs.Domain = domainWithScheme[len("http://"):]
    } else if strings.HasPrefix(domainWithScheme, "https://") {
        addrs.Domain = domainWithScheme[len("https://"):]
        if this.Cert == "" {
            return nil, fmt.Errorf("env var ISS_CERT required for https")
        }
        addrs.CertKey = path.Join(this.Cert, "cert.key")
        addrs.CertPem = path.Join(this.Cert, "cert.pem")
    } else {
        return nil, fmt.Errorf("invalid URL scheme. Only http and https allowed.")
    }

    redirectUrlBuilder.WriteString("/api/auth/cb")
    addrs.RedirectUrl = redirectUrlBuilder.String()

    DrawSection("Server Address")
    DrawLine("Origin", addrs.Origin)
    DrawLine("Domain", addrs.Domain)
    DrawLine("Redirect URL", addrs.RedirectUrl)
    DrawLine("Port", addrs.Port)

    return &addrs, nil
}

func (this *EnvAddress) IsHttps() bool {
    return this.CertKey != "" && this.CertPem != ""
}
