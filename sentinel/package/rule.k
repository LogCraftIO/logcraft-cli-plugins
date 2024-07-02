schema Rule:
    """Represent scheduled alert rule.

    Attributes
    ----------
    kind: str, 'Scheduled', required
        The alert rule kind.

    properties: Properties, required
        The display name for alerts created by this alert rule.

    etag: str, optional
        Etag of the azure resource
    ruleId: str, optional
        ID of the rule. The global rule name is used if not specified.
    """
    # Mandatory
    kind: str = "Scheduled"
    properties: Properties
    
    # Optional
    etag?: str
    ruleId?: str


schema Properties:
    """
    enabled: bool, required
        Determines whether this alert rule is enabled or disabled.

    query: str, required
        The query that creates alerts for this rule.

    queryFrequency: str, required
        The frequency (in ISO 8601 duration format) for this alert rule to run.

    queryPeriod: str, required
        The period (in ISO 8601 duration format) that this alert rule looks at.

    severity: AlertSeverity, required
        The severity for alerts created by this alert rule.

    suppressionDuration: str, required
        The suppression (in ISO 8601 duration format) to wait since last time this alert rule been triggered.

    suppressionEnabled: bool, required
        Determines whether the suppression for this alert rule is enabled or disabled.

    triggerOperator: TriggerOperator, required
        The operation against the threshold that triggers alert rule.

    triggerThreshold: int, required
        The threshold triggers this alert rule.

    alertDetailsOverride: AlertDetailsOverride
        The alert details override settings

    alertRuleTemplateName: str
        The Name of the alert rule template used to create this rule.

    customDetails: object
        Dictionary of string key-value pairs of columns to be attached to the alert

    description: str
        The description of the alert rule.

    entityMappings: [entityMappings] = []
        Array of the entity mappings of the alert rule

    eventGroupingSettings: eventGroupingSettings
        The event grouping settings.

    incidentConfiguration: incidentConfiguration
        The settings of the incidents that created from alerts triggered by this analytics rule

    tactics: [AttackTactic]
        The tactics of the alert rule

    techniques: str[]
        The techniques of the alert rule

    templateVersion: str
        The version of the alert rule template used to create this rule - in format <a.b.c>, where all are numbers, for example 0 <1.0.2>

    displayName: str, optional
        The display name for alerts created by this alert rule. The global rule name is used if not specified.
    """
    # Mandatory Parameters
    severity: AlertSeverity
    query: str
    queryFrequency: str = "PT5H"
    queryPeriod: str = "PT5H"
    suppressionDuration: str = "PT5H"
    suppressionEnabled: bool = False
    enabled: bool = True
    triggerOperator: TriggerOperator = "GreaterThan"
    triggerThreshold: int = 0

    # Optional Parameters
    alertDetailsOverride?: AlertDetailsOverride
    alertRuleTemplateName?: str
    customDetails?: {str: any}
    description?: str
    entityMappings?: [EntityMapping] = []
    eventGroupingSettings?: EventGroupingSettings
    incidentConfiguration?: IncidentConfiguration
    tactics?: [AttackTactic]
    techniques?: [str]
    templateVersion?: str
    displayName?: str

type AlertSeverity = "High" | "Informational" | "Low" | "Medium"

type AttackTactic = "Collection" | "CommandAndControl" | "CredentialAccess" | "DefenseEvasion" | "Discovery" | "Execution" | "Exfiltration" | "Impact" | "ImpairProcessControl" | "InhibitResponseFunction" | "InitialAccess" | "LateralMovement" | "Persistence" | "PreAttack" | "PrivilegeEscalation" | "Reconnaissance" | "ResourceDevelopment"

type TriggerOperator = "Equal" | "GreaterThan" | "LessThan" | "NotEqual"

schema AlertDetailsOverride:
    """
    alertDescriptionFormat: str
        The format containing columns name(s) to override the alert description
    alertDisplayNameFormat: str
        The format containing columns name(s) to override the alert name
    alertDynamicProperties: [AlertPropertyMapping]
        List of additional dynamic properties to override
    alertSeverityColumnName: str
        The column name to take the alert severity from
    alertTacticsColumnName: str
        The column name to take the alert tactics from
    """
    alertDescriptionFormat: str
    alertDisplayNameFormat: str
    alertDynamicProperties: [AlertPropertyMapping] = []
    alertSeverityColumnName: str
    alertTacticsColumnName: str
    

type AlertProperty = "AlertLink" | "ConfidenceLevel" | "ConfidenceScore" | "ExtendedLinks" | "ProductComponentName" | "ProductName" | "ProviderName" | "RemediationSteps" | "Techniques"

schema AlertPropertyMapping:
    """
    Attributes
    ----------
    alertProperty: AlertProperty, required
        The V3 alert property
    value: str, required
        The column name to use to override this property
    """
    alertProperty: AlertProperty
    value: str


type EntityMappingType = "Account" | "AzureResource" | "CloudApplication" | "DNS" | "File" | "FileHash" | "Host" | "IP" | "MailCluster" | "MailMessage" | "Mailbox" | "Malware" | "Process" | "RegistryKey" | "RegistryValue" | "SecurityGroup" | "SubmissionMail" | "URL"

schema FieldMapping:
    """
    Attributes
    ----------
    columnName: str, required
        The column name to be mapped to the identifier
    identifier: str, required
        The V3 identifier of the entity

    """
    columnName: str
    identifier: str

schema EntityMapping:
    """
    Attributes
    ----------
    entityType: EntityMappingType, required
        The V3 alert property
    fieldMappings: [FieldMapping], required
        The column name to use to override this property
    """
    entityType: EntityMappingType
    fieldMappings: [FieldMapping]


schema EventGroupingAggregationKind:
    """
    Attributes
    ----------
    AlertPerResult: str, required
    SingleAlert: str, required
    """
    AlertPerResult: str
    SingleAlert: str

schema EventGroupingSettings:
    """
    Attributes
    ----------
    aggregationKind: EventGroupingAggregationKind, required
        The event grouping aggregation kinds
    """
    aggregationKind: EventGroupingAggregationKind

schema IncidentConfiguration:
    """
    Attributes
    ----------
    createIncident: bool, required
        Create incidents from alerts triggered by this analytics rule
    groupingConfiguration: GroupingConfiguration, required
        Set how the alerts that are triggered by this analytics rule, are grouped into incidents
    """
    createIncident: bool
    groupingConfiguration: GroupingConfiguration

schema GroupingConfiguration:
    """
    Attributes
    ----------
    enabled: bool, required
        Grouping enabled
    groupByAlertDetails: [AlertDetail] = []
        A list of alert details to group by (when matchingMethod is Selected)
    groupByCustomDetails: [str] = []
        A list of custom details keys to group by (when matchingMethod is Selected). Only keys defined in the current alert rule may be used.
    groupByEntities: [EntityMappingType] = []
        A list of entity types to group by (when matchingMethod is Selected). Only entities defined in the current alert rule may be used.
    lookbackDuration: str
        Limit the group to alerts created within the lookback duration (in ISO 8601 duration format)
    matchingMethod: MatchingMethod
        Grouping matching method. When method is Selected at least one of groupByEntities, groupByAlertDetails, groupByCustomDetails must be provided and not empty.
    reopenClosedIncident: bool
        Re-open closed matching incidents
    """
    enabled: bool
    groupByAlertDetails: [AlertDetail] = []
    groupByCustomDetails: [str] = []
    groupByEntities: [EntityMappingType] = []
    lookbackDuration: str
    matchingMethod: MatchingMethod
    reopenClosedIncident: bool

type MatchingMethod = "AllEntities" | "AnyAlert" | "Selected"

schema AlertDetail:
    """
    Attributes
    ----------
    displayName: str, required
        Alert display name
    Severity: AlertSeverity, required
        Alert severity
    """
    displayName: str
    Severity: AlertSeverity