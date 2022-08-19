package io.drogue.iot.hackathon.registry;

import com.fasterxml.jackson.databind.ObjectMapper;
import io.quarkus.runtime.Startup;
import org.eclipse.microprofile.config.inject.ConfigProperty;
import org.eclipse.microprofile.rest.client.inject.RestClient;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.enterprise.context.ApplicationScoped;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

@Startup
@ApplicationScoped
public class Registry {
    private static final Logger LOG = LoggerFactory.getLogger(Registry.class);

    @ConfigProperty(name = "drogue.application.name")
    String applicationName;

    @RestClient
    RegistryService registryService;

    public void createDevice(String device, String[] aliases) {
        // List gateways
        List<String> gateways = new ArrayList<>();
        List<Device> devices = registryService.getDevices(applicationName, "role=gateway");
        if (devices != null) {
            for (Device gateway : devices) {
                gateways.add(gateway.getMetadata().getName());
            }
        }

        LOG.info("Using gateways {}", gateways);

        // Create device struct
        Device dev = new Device();
        Metadata metadata = new Metadata();
        metadata.setName(device);
        metadata.setApplication(applicationName);
        dev.setMetadata(metadata);

        DeviceSpec spec = new DeviceSpec();
        DeviceAliases deviceAliases = new DeviceAliases();
        deviceAliases.setAliases(Arrays.asList(aliases));
        spec.setAlias(deviceAliases);

        GatewaySelector selector = new GatewaySelector();
        selector.setMatchNames(gateways);
        spec.setSelector(selector);

        dev.setSpec(spec);

        try {
            LOG.info("Creating device: {}", new ObjectMapper().writeValueAsString(dev));
        } catch (Exception e) {
            // Ignored
        }

        // Post device
        registryService.createDevice(applicationName, dev);
    }
}