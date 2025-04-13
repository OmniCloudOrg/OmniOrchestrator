# Planned routes for OmniOrchestrator

---
## Dashboard homepage

### Top Cards
- Total applications
- Total Running Instances
- Avg platform CPU usage
- Monthly total platform cost
- Platform health statistics
- API Gateway status (To be replaced with Proxy health)
- Database clusters health (Omni's internal DB and all DBs it manages)

### Multi-cloud Status
- Region name
- Provider name (Multiple providers can make up a single region)
- Provider-region status (The status of a given provider in a given region)
- Apps (Total apps on a given provider in a given region)
- Instances (Total app-instances on a given provider in a given region)

### Resource Usage
Average usage for each resource across all providers and regions (these are a percentage usage of your total cross-provider quota, we will show a dotted line where reserved resources end and on-demand begins)
- CPU
- Memory
- Disk
- Network

### Running Services
A list of all services (Applications or App stacks) running across all regions and providers.

- Service (Name of the service)
- Status (Healthy, Maintainance, Warning, Critical)
- Instances (number of running instances of the app or stack) (To be replaced with scale multiplier)
- CPU (CPU usage of the service averaged across all instance)
- Memory (Memory usage of the service averaged across all instance)
- Provider (The cloud provider(s) tasked with running instances of this service)

### Cost Overview
A chart-based cost breakdown showing what providers cost you the most (we be able to be normalized against load for proper cost analysis)

- Pie Chart (A visual cost comparison averages over the selected period)
- Bar Graph (A change-over-time chart showing evolution of total cost and cost across providers over the selected period)

### Active Alerts
A list of any active alerts to your platform (from your services or internal platform components)

### Recent builds
A list of recent builds of your services, container images, and VM images across all platforms.

### Recent Activity
Recent Activity a summery of all recent audit items at info level or higher

---

### Applications Page
A searchable, filterable grid of all applications, and stacks running on your platform.

#### Application -> Overview
A tab showing basic information about the app for easy access.

##### Top Cards

- Uptime (Percentage of uptime for the app)
- Response time (Measured and reported by Lodestone this is the average time a response of any kind takes the app)
- Error rate (Measured and reported by Lodestone this is the percentage of the time the app returned an error in the 500 range)
- Deployments (To be replaced with instances, this is the current instance count across alll regions and providers)

##### Application Details

- Version (An insternally generated or user provided version number for the app)
- Runtime (What runtime(s) if any does the app require, we detect this at deploy time when building the runtime container image)
- Region (The region(s) the app runs instances in)
- Created (THe first time Omni saw this app)
- Last Update (The last time the app version number bumped)
- Repository (if applicable this links to the app's src repo)

##### Environment Variables
All omni-or-user-defined environment variables (These are availible to all instances of the app, but can be overridden at runtime by programs in the app container)

##### Recent Activity
A list of all recent audi items at level info or above related to this app.

#### Application -> Instances
A tab showing more detailed information about the instance of a given app.

##### Instances
A paginsted list of the app's instances.
- Instance ID (The machine-generated ID for the instance)
- Status (Instance status can be any of: Healthy, Maintainance, Warning, Critical)
- Region (the region this app instance runs in)
- CPU (the CPU usage of this instance)
- Memory (the Memory usage of this instance)
- Uptime (The uptime of this instance)

###### Controls
- Delete (X)
- Restart (Reload arrows)
- Shell (Terminal icon)

##### Auto Scaling Configuration
A referance card on the instances page showing auto-scaling configuration info. We will add a go to settings link here soon

##### Health Checks
A referance card on the instances page showing health check configuration info. We will add a go to settings link here soon

#### Application -> Deployments
A list of all deployments, some basic metadata we have on-hand, and a details button


#### Application -> Logs
Yup, just a color-coded logs tab. That's it.

#### Application -> Metrics

##### Performance metrics
Charts showing Application total resource usage vs quota (we will draw a dotted line where reserved resources end and on-demand capacity begins)

##### Top Routes
The most visited routes underneath this app's routes list (Collected by Lodestone)

##### Status Codes
The most returned status codes by the app (assuming it uses HTTP). (This is collected by Lodestone)

##### Cache Hit Ratio
If applicable this card tracks the ammount of requests that hit the cach vs the ones allowed past the cache to the real app.

#### Application -> Settings
The apps settings tab

---

