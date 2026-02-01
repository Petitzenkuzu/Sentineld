# Sentineld

## English

### What the agent does
- Exposes Prometheus metrics for VM/system monitoring.
- Collects CPU, memory, disk, and agent uptime metrics.
- Serves HTTP endpoints: `/metrics` for Prometheus and `/health` for a simple health check.
- Supports hot reload of `config.yml` (keeps the last valid config if reload fails and increments the error counter).

### How to use it
By default, the agent reads `./config.yml`. You can override the path with the `SENTINELD_CONFIG` environment variable.

PowerShell example:
```
$env:SENTINELD_CONFIG="C:\path\to\config.yml"
.\sentineld.exe
```

Linux/macOS example:
```
SENTINELD_CONFIG=/path/to/config.yml ./sentineld
```

### Configuration
The default config file looks like this:
```
port: 9100
host: "0.0.0.0"
exporter:
  cpu_interval: 5
  memory_interval: 5
  disk_interval: 5
  agent_interval: 5
metrics:
  cpu: true
  memory: true
  disk: true
  agent: true
collection:
  interval: 5
```

Notes:
- `host`/`port` define the HTTP bind address (default `0.0.0.0:9100`).
- `exporter.*_interval` controls how often each metric family is exported.
- `collection.interval` controls how often the system snapshot is refreshed.
- `metrics.*` toggles each metric family on/off.

Prometheus metrics include (names are stable in code):
- `sentinel_cpu_usage_percent`
- `sentinel_memory_used_bytes`, `sentinel_memory_total_bytes`, `sentinel_memory_free_bytes`
- `sentinel_disk_total_bytes`, `sentinel_disk_free_bytes`, `sentinel_disk_used_bytes`
- `sentinel_agent_uptime_seconds`
- `sentinel_agent_errors_count` (increments on config reload failure)

## Francais

### Ce que fait l'agent
- Expose des metriques Prometheus pour monitorer une VM/systeme.
- Collecte CPU, memoire, disque et uptime de l'agent.
- Sert les endpoints HTTP: `/metrics` (Prometheus) et `/health` (health check).
- Hot reload du `config.yml` (garde la derniere config valide si le reload echoue et incremente le compteur d'erreurs).

### Comment l'utiliser
Par defaut, l'agent lit `./config.yml`. Tu peux surcharger le chemin avec la variable d'environnement `SENTINELD_CONFIG`.

Exemple PowerShell:
```
$env:SENTINELD_CONFIG="C:\path\to\config.yml"
.\sentineld.exe
```

Exemple Linux/macOS:
```
SENTINELD_CONFIG=/path/to/config.yml ./sentineld
```

### Configuration
Le fichier de config par defaut:
```
port: 9100
host: "0.0.0.0"
exporter:
  cpu_interval: 5
  memory_interval: 5
  disk_interval: 5
  agent_interval: 5
metrics:
  cpu: true
  memory: true
  disk: true
  agent: true
collection:
  interval: 5
```

Notes:
- `host`/`port` definissent l'adresse d'ecoute HTTP (par defaut `0.0.0.0:9100`).
- `exporter.*_interval` controle la frequence d'export des metriques.
- `collection.interval` controle la frequence de refresh du snapshot systeme.
- `metrics.*` active/desactive chaque famille de metriques.

Metriques Prometheus exposees (noms stables dans le code):
- `sentinel_cpu_usage_percent`
- `sentinel_memory_used_bytes`, `sentinel_memory_total_bytes`, `sentinel_memory_free_bytes`
- `sentinel_disk_total_bytes`, `sentinel_disk_free_bytes`, `sentinel_disk_used_bytes`
- `sentinel_agent_uptime_seconds`
- `sentinel_agent_errors_count` (incremente en cas d'echec de reload)
