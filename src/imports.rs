pub static LIBS: &[(&str, &[u8])] = &[
	(r#"x64/ACLUI.lib"#, X64_ACLUI),
	(r#"x64/ACTIVEDS.lib"#, X64_ACTIVEDS),
	(r#"x64/ADVAPI32.lib"#, X64_ADVAPI32),
	(r#"x64/ADVPACK.lib"#, X64_ADVPACK),
	(r#"x64/Amsi.lib"#, X64_AMSI),
	(r#"x64/api-ms-win-appmodel-runtime-l1-1-1.lib"#, X64_API_MS_WIN_APPMODEL_RUNTIME_L1_1_1),
	(r#"x64/api-ms-win-appmodel-runtime-l1-1-3.lib"#, X64_API_MS_WIN_APPMODEL_RUNTIME_L1_1_3),
	(r#"x64/api-ms-win-core-apiquery-l2-1-0.lib"#, X64_API_MS_WIN_CORE_APIQUERY_L2_1_0),
	(r#"x64/api-ms-win-core-backgroundtask-l1-1-0.lib"#, X64_API_MS_WIN_CORE_BACKGROUNDTASK_L1_1_0),
	(r#"x64/api-ms-win-core-comm-l1-1-1.lib"#, X64_API_MS_WIN_CORE_COMM_L1_1_1),
	(r#"x64/api-ms-win-core-comm-l1-1-2.lib"#, X64_API_MS_WIN_CORE_COMM_L1_1_2),
	(r#"x64/api-ms-win-core-errorhandling-l1-1-3.lib"#, X64_API_MS_WIN_CORE_ERRORHANDLING_L1_1_3),
	(r#"x64/api-ms-win-core-featurestaging-l1-1-0.lib"#, X64_API_MS_WIN_CORE_FEATURESTAGING_L1_1_0),
	(r#"x64/api-ms-win-core-featurestaging-l1-1-1.lib"#, X64_API_MS_WIN_CORE_FEATURESTAGING_L1_1_1),
	(r#"x64/api-ms-win-core-file-fromapp-l1-1-0.lib"#, X64_API_MS_WIN_CORE_FILE_FROMAPP_L1_1_0),
	(r#"x64/api-ms-win-core-handle-l1-1-0.lib"#, X64_API_MS_WIN_CORE_HANDLE_L1_1_0),
	(r#"x64/api-ms-win-core-libraryloader-l2-1-0.lib"#, X64_API_MS_WIN_CORE_LIBRARYLOADER_L2_1_0),
	(r#"x64/api-ms-win-core-memory-l1-1-3.lib"#, X64_API_MS_WIN_CORE_MEMORY_L1_1_3),
	(r#"x64/api-ms-win-core-memory-l1-1-4.lib"#, X64_API_MS_WIN_CORE_MEMORY_L1_1_4),
	(r#"x64/api-ms-win-core-memory-l1-1-5.lib"#, X64_API_MS_WIN_CORE_MEMORY_L1_1_5),
	(r#"x64/api-ms-win-core-memory-l1-1-6.lib"#, X64_API_MS_WIN_CORE_MEMORY_L1_1_6),
	(r#"x64/api-ms-win-core-memory-l1-1-7.lib"#, X64_API_MS_WIN_CORE_MEMORY_L1_1_7),
	(r#"x64/api-ms-win-core-path-l1-1-0.lib"#, X64_API_MS_WIN_CORE_PATH_L1_1_0),
	(r#"x64/api-ms-win-core-psm-appnotify-l1-1-0.lib"#, X64_API_MS_WIN_CORE_PSM_APPNOTIFY_L1_1_0),
	(r#"x64/api-ms-win-core-psm-appnotify-l1-1-1.lib"#, X64_API_MS_WIN_CORE_PSM_APPNOTIFY_L1_1_1),
	(r#"x64/api-ms-win-core-realtime-l1-1-1.lib"#, X64_API_MS_WIN_CORE_REALTIME_L1_1_1),
	(r#"x64/api-ms-win-core-realtime-l1-1-2.lib"#, X64_API_MS_WIN_CORE_REALTIME_L1_1_2),
	(r#"x64/api-ms-win-core-slapi-l1-1-0.lib"#, X64_API_MS_WIN_CORE_SLAPI_L1_1_0),
	(r#"x64/api-ms-win-core-state-helpers-l1-1-0.lib"#, X64_API_MS_WIN_CORE_STATE_HELPERS_L1_1_0),
	(r#"x64/api-ms-win-core-sysinfo-l1-2-0.lib"#, X64_API_MS_WIN_CORE_SYSINFO_L1_2_0),
	(r#"x64/api-ms-win-core-sysinfo-l1-2-3.lib"#, X64_API_MS_WIN_CORE_SYSINFO_L1_2_3),
	(r#"x64/api-ms-win-core-sysinfo-l1-2-4.lib"#, X64_API_MS_WIN_CORE_SYSINFO_L1_2_4),
	(r#"x64/api-ms-win-core-util-l1-1-1.lib"#, X64_API_MS_WIN_CORE_UTIL_L1_1_1),
	(r#"x64/api-ms-win-core-winrt-error-l1-1-0.lib"#, X64_API_MS_WIN_CORE_WINRT_ERROR_L1_1_0),
	(r#"x64/api-ms-win-core-winrt-error-l1-1-1.lib"#, X64_API_MS_WIN_CORE_WINRT_ERROR_L1_1_1),
	(r#"x64/api-ms-win-core-winrt-l1-1-0.lib"#, X64_API_MS_WIN_CORE_WINRT_L1_1_0),
	(r#"x64/api-ms-win-core-winrt-registration-l1-1-0.lib"#, X64_API_MS_WIN_CORE_WINRT_REGISTRATION_L1_1_0),
	(r#"x64/api-ms-win-core-winrt-robuffer-l1-1-0.lib"#, X64_API_MS_WIN_CORE_WINRT_ROBUFFER_L1_1_0),
	(r#"x64/api-ms-win-core-winrt-roparameterizediid-l1-1-0.lib"#, X64_API_MS_WIN_CORE_WINRT_ROPARAMETERIZEDIID_L1_1_0),
	(r#"x64/api-ms-win-core-winrt-string-l1-1-0.lib"#, X64_API_MS_WIN_CORE_WINRT_STRING_L1_1_0),
	(r#"x64/api-ms-win-core-winrt-string-l1-1-1.lib"#, X64_API_MS_WIN_CORE_WINRT_STRING_L1_1_1),
	(r#"x64/api-ms-win-core-wow64-l1-1-1.lib"#, X64_API_MS_WIN_CORE_WOW64_L1_1_1),
	(r#"x64/api-ms-win-devices-query-l1-1-0.lib"#, X64_API_MS_WIN_DEVICES_QUERY_L1_1_0),
	(r#"x64/api-ms-win-devices-query-l1-1-1.lib"#, X64_API_MS_WIN_DEVICES_QUERY_L1_1_1),
	(r#"x64/api-ms-win-dx-d3dkmt-l1-1-0.lib"#, X64_API_MS_WIN_DX_D3DKMT_L1_1_0),
	(r#"x64/api-ms-win-gaming-deviceinformation-l1-1-0.lib"#, X64_API_MS_WIN_GAMING_DEVICEINFORMATION_L1_1_0),
	(r#"x64/api-ms-win-gaming-expandedresources-l1-1-0.lib"#, X64_API_MS_WIN_GAMING_EXPANDEDRESOURCES_L1_1_0),
	(r#"x64/api-ms-win-gaming-tcui-l1-1-0.lib"#, X64_API_MS_WIN_GAMING_TCUI_L1_1_0),
	(r#"x64/api-ms-win-gaming-tcui-l1-1-1.lib"#, X64_API_MS_WIN_GAMING_TCUI_L1_1_1),
	(r#"x64/api-ms-win-gaming-tcui-l1-1-2.lib"#, X64_API_MS_WIN_GAMING_TCUI_L1_1_2),
	(r#"x64/api-ms-win-gaming-tcui-l1-1-3.lib"#, X64_API_MS_WIN_GAMING_TCUI_L1_1_3),
	(r#"x64/api-ms-win-gaming-tcui-l1-1-4.lib"#, X64_API_MS_WIN_GAMING_TCUI_L1_1_4),
	(r#"x64/api-ms-win-mm-misc-l1-1-1.lib"#, X64_API_MS_WIN_MM_MISC_L1_1_1),
	(r#"x64/api-ms-win-net-isolation-l1-1-0.lib"#, X64_API_MS_WIN_NET_ISOLATION_L1_1_0),
	(r#"x64/api-ms-win-security-base-l1-2-2.lib"#, X64_API_MS_WIN_SECURITY_BASE_L1_2_2),
	(r#"x64/api-ms-win-security-isolatedcontainer-l1-1-0.lib"#, X64_API_MS_WIN_SECURITY_ISOLATEDCONTAINER_L1_1_0),
	(r#"x64/api-ms-win-security-isolatedcontainer-l1-1-1.lib"#, X64_API_MS_WIN_SECURITY_ISOLATEDCONTAINER_L1_1_1),
	(r#"x64/api-ms-win-service-core-l1-1-3.lib"#, X64_API_MS_WIN_SERVICE_CORE_L1_1_3),
	(r#"x64/api-ms-win-service-core-l1-1-4.lib"#, X64_API_MS_WIN_SERVICE_CORE_L1_1_4),
	(r#"x64/api-ms-win-shcore-scaling-l1-1-0.lib"#, X64_API_MS_WIN_SHCORE_SCALING_L1_1_0),
	(r#"x64/api-ms-win-shcore-scaling-l1-1-1.lib"#, X64_API_MS_WIN_SHCORE_SCALING_L1_1_1),
	(r#"x64/api-ms-win-shcore-scaling-l1-1-2.lib"#, X64_API_MS_WIN_SHCORE_SCALING_L1_1_2),
	(r#"x64/api-ms-win-shcore-stream-winrt-l1-1-0.lib"#, X64_API_MS_WIN_SHCORE_STREAM_WINRT_L1_1_0),
	(r#"x64/APPHELP.lib"#, X64_APPHELP),
	(r#"x64/AUTHZ.lib"#, X64_AUTHZ),
	(r#"x64/AVICAP32.lib"#, X64_AVICAP32),
	(r#"x64/AVIFIL32.lib"#, X64_AVIFIL32),
	(r#"x64/bcrypt.lib"#, X64_BCRYPT),
	(r#"x64/BluetoothApis.lib"#, X64_BLUETOOTHAPIS),
	(r#"x64/bthprops.lib"#, X64_BTHPROPS),
	(r#"x64/Cabinet.lib"#, X64_CABINET),
	(r#"x64/certadm.lib"#, X64_CERTADM),
	(r#"x64/certpoleng.lib"#, X64_CERTPOLENG),
	(r#"x64/CFGMGR32.lib"#, X64_CFGMGR32),
	(r#"x64/chakra.lib"#, X64_CHAKRA),
	(r#"x64/cldapi.lib"#, X64_CLDAPI),
	(r#"x64/clfsw32.lib"#, X64_CLFSW32),
	(r#"x64/CLUSAPI.lib"#, X64_CLUSAPI),
	(r#"x64/COMCTL32.lib"#, X64_COMCTL32),
	(r#"x64/COMDLG32.lib"#, X64_COMDLG32),
	(r#"x64/comsvcs.lib"#, X64_COMSVCS),
	(r#"x64/CoreMessaging.lib"#, X64_COREMESSAGING),
	(r#"x64/credui.lib"#, X64_CREDUI),
	(r#"x64/CRYPT32.lib"#, X64_CRYPT32),
	(r#"x64/CRYPTNET.lib"#, X64_CRYPTNET),
	(r#"x64/CRYPTUI.lib"#, X64_CRYPTUI),
	(r#"x64/CRYPTXML.lib"#, X64_CRYPTXML),
	(r#"x64/CSCAPI.lib"#, X64_CSCAPI),
	(r#"x64/d2d1.lib"#, X64_D2D1),
	(r#"x64/d3d10.lib"#, X64_D3D10),
	(r#"x64/d3d10_1.lib"#, X64_D3D10_1),
	(r#"x64/d3d11.lib"#, X64_D3D11),
	(r#"x64/d3d12.lib"#, X64_D3D12),
	(r#"x64/d3d9.lib"#, X64_D3D9),
	(r#"x64/D3DCOMPILER_47.lib"#, X64_D3DCOMPILER_47),
	(r#"x64/d3dcsx.lib"#, X64_D3DCSX),
	(r#"x64/davclnt.lib"#, X64_DAVCLNT),
	(r#"x64/dbgeng.lib"#, X64_DBGENG),
	(r#"x64/dbghelp.lib"#, X64_DBGHELP),
	(r#"x64/dbgmodel.lib"#, X64_DBGMODEL),
	(r#"x64/DCIMAN32.lib"#, X64_DCIMAN32),
	(r#"x64/dcomp.lib"#, X64_DCOMP),
	(r#"x64/DDRAW.lib"#, X64_DDRAW),
	(r#"x64/deviceaccess.lib"#, X64_DEVICEACCESS),
	(r#"x64/dflayout.lib"#, X64_DFLAYOUT),
	(r#"x64/dhcpcsvc.lib"#, X64_DHCPCSVC),
	(r#"x64/dhcpcsvc6.lib"#, X64_DHCPCSVC6),
	(r#"x64/DHCPSAPI.lib"#, X64_DHCPSAPI),
	(r#"x64/DiagnosticDataQuery.lib"#, X64_DIAGNOSTICDATAQUERY),
	(r#"x64/DINPUT8.lib"#, X64_DINPUT8),
	(r#"x64/DirectML.lib"#, X64_DIRECTML),
	(r#"x64/DNSAPI.lib"#, X64_DNSAPI),
	(r#"x64/drt.lib"#, X64_DRT),
	(r#"x64/drtprov.lib"#, X64_DRTPROV),
	(r#"x64/drttransport.lib"#, X64_DRTTRANSPORT),
	(r#"x64/DSOUND.lib"#, X64_DSOUND),
	(r#"x64/DSPARSE.lib"#, X64_DSPARSE),
	(r#"x64/dsprop.lib"#, X64_DSPROP),
	(r#"x64/DSSEC.lib"#, X64_DSSEC),
	(r#"x64/dsuiext.lib"#, X64_DSUIEXT),
	(r#"x64/dwmapi.lib"#, X64_DWMAPI),
	(r#"x64/DWrite.lib"#, X64_DWRITE),
	(r#"x64/dxgi.lib"#, X64_DXGI),
	(r#"x64/dxva2.lib"#, X64_DXVA2),
	(r#"x64/eappcfg.lib"#, X64_EAPPCFG),
	(r#"x64/eappprxy.lib"#, X64_EAPPPRXY),
	(r#"x64/efswrt.lib"#, X64_EFSWRT),
	(r#"x64/elscore.lib"#, X64_ELSCORE),
	(r#"x64/ESENT.lib"#, X64_ESENT),
	(r#"x64/EVR.lib"#, X64_EVR),
	(r#"x64/faultrep.lib"#, X64_FAULTREP),
	(r#"x64/fhsvcctl.lib"#, X64_FHSVCCTL),
	(r#"x64/FLTLIB.lib"#, X64_FLTLIB),
	(r#"x64/FONTSUB.lib"#, X64_FONTSUB),
	(r#"x64/fwpuclnt.lib"#, X64_FWPUCLNT),
	(r#"x64/fxsutility.lib"#, X64_FXSUTILITY),
	(r#"x64/GDI32.lib"#, X64_GDI32),
	(r#"x64/GPEDIT.lib"#, X64_GPEDIT),
	(r#"x64/HID.lib"#, X64_HID),
	(r#"x64/hlink.lib"#, X64_HLINK),
	(r#"x64/HrtfApo.lib"#, X64_HRTFAPO),
	(r#"x64/HTTPAPI.lib"#, X64_HTTPAPI),
	(r#"x64/ICM32.lib"#, X64_ICM32),
	(r#"x64/ICMUI.lib"#, X64_ICMUI),
	(r#"x64/icu.lib"#, X64_ICU),
	(r#"x64/IMM32.lib"#, X64_IMM32),
	(r#"x64/inkobjcore.lib"#, X64_INKOBJCORE),
	(r#"x64/IPHLPAPI.lib"#, X64_IPHLPAPI),
	(r#"x64/ISCSIDSC.lib"#, X64_ISCSIDSC),
	(r#"x64/KERNEL32.lib"#, X64_KERNEL32),
	(r#"x64/KeyCredMgr.lib"#, X64_KEYCREDMGR),
	(r#"x64/ksuser.lib"#, X64_KSUSER),
	(r#"x64/ktmw32.lib"#, X64_KTMW32),
	(r#"x64/loadperf.lib"#, X64_LOADPERF),
	(r#"x64/MAGNIFICATION.lib"#, X64_MAGNIFICATION),
	(r#"x64/MAPI32.lib"#, X64_MAPI32),
	(r#"x64/MDMRegistration.lib"#, X64_MDMREGISTRATION),
	(r#"x64/MF.lib"#, X64_MF),
	(r#"x64/MFCORE.lib"#, X64_MFCORE),
	(r#"x64/MFPlat.lib"#, X64_MFPLAT),
	(r#"x64/MFPlay.lib"#, X64_MFPLAY),
	(r#"x64/MFReadWrite.lib"#, X64_MFREADWRITE),
	(r#"x64/MFSENSORGROUP.lib"#, X64_MFSENSORGROUP),
	(r#"x64/mfsrcsnk.lib"#, X64_MFSRCSNK),
	(r#"x64/mgmtapi.lib"#, X64_MGMTAPI),
	(r#"x64/mi.lib"#, X64_MI),
	(r#"x64/MMDevAPI.lib"#, X64_MMDEVAPI),
	(r#"x64/MPR.lib"#, X64_MPR),
	(r#"x64/MPRAPI.lib"#, X64_MPRAPI),
	(r#"x64/MrmSupport.lib"#, X64_MRMSUPPORT),
	(r#"x64/MSACM32.lib"#, X64_MSACM32),
	(r#"x64/MSAJApi.lib"#, X64_MSAJAPI),
	(r#"x64/mscms.lib"#, X64_MSCMS),
	(r#"x64/MsCtfMonitor.lib"#, X64_MSCTFMONITOR),
	(r#"x64/msdmo.lib"#, X64_MSDMO),
	(r#"x64/msdrm.lib"#, X64_MSDRM),
	(r#"x64/msi.lib"#, X64_MSI),
	(r#"x64/MSIMG32.lib"#, X64_MSIMG32),
	(r#"x64/MSPORTS.lib"#, X64_MSPORTS),
	(r#"x64/mstask.lib"#, X64_MSTASK),
	(r#"x64/MSVFW32.lib"#, X64_MSVFW32),
	(r#"x64/MSWSOCK.lib"#, X64_MSWSOCK),
	(r#"x64/MTxDM.lib"#, X64_MTXDM),
	(r#"x64/ncrypt.lib"#, X64_NCRYPT),
	(r#"x64/NDFAPI.lib"#, X64_NDFAPI),
	(r#"x64/NETAPI32.lib"#, X64_NETAPI32),
	(r#"x64/NETSH.lib"#, X64_NETSH),
	(r#"x64/newdev.lib"#, X64_NEWDEV),
	(r#"x64/NInput.lib"#, X64_NINPUT),
	(r#"x64/NORMALIZ.lib"#, X64_NORMALIZ),
	(r#"x64/ntdll.lib"#, X64_NTDLL),
	(r#"x64/NTDSAPI.lib"#, X64_NTDSAPI),
	(r#"x64/NTLANMAN.lib"#, X64_NTLANMAN),
	(r#"x64/OLE32.lib"#, X64_OLE32),
	(r#"x64/OLEACC.lib"#, X64_OLEACC),
	(r#"x64/OLEAUT32.lib"#, X64_OLEAUT32),
	(r#"x64/oledlg.lib"#, X64_OLEDLG),
	(r#"x64/OnDemandConnRouteHelper.lib"#, X64_ONDEMANDCONNROUTEHELPER),
	(r#"x64/OPENGL32.lib"#, X64_OPENGL32),
	(r#"x64/P2P.lib"#, X64_P2P),
	(r#"x64/P2PGRAPH.lib"#, X64_P2PGRAPH),
	(r#"x64/pdh.lib"#, X64_PDH),
	(r#"x64/PeerDist.lib"#, X64_PEERDIST),
	(r#"x64/POWRPROF.lib"#, X64_POWRPROF),
	(r#"x64/prntvpt.lib"#, X64_PRNTVPT),
	(r#"x64/PROJECTEDFSLIB.lib"#, X64_PROJECTEDFSLIB),
	(r#"x64/PROPSYS.lib"#, X64_PROPSYS),
	(r#"x64/QUARTZ.lib"#, X64_QUARTZ),
	(r#"x64/query.lib"#, X64_QUERY),
	(r#"x64/qwave.lib"#, X64_QWAVE),
	(r#"x64/RASAPI32.lib"#, X64_RASAPI32),
	(r#"x64/RASDLG.lib"#, X64_RASDLG),
	(r#"x64/RESUTILS.lib"#, X64_RESUTILS),
	(r#"x64/RoMetadata.lib"#, X64_ROMETADATA),
	(r#"x64/RPCNS4.lib"#, X64_RPCNS4),
	(r#"x64/RPCRT4.lib"#, X64_RPCRT4),
	(r#"x64/RstrtMgr.lib"#, X64_RSTRTMGR),
	(r#"x64/rtm.lib"#, X64_RTM),
	(r#"x64/SAS.lib"#, X64_SAS),
	(r#"x64/SCARDDLG.lib"#, X64_SCARDDLG),
	(r#"x64/SCHANNEL.lib"#, X64_SCHANNEL),
	(r#"x64/SECUR32.lib"#, X64_SECUR32),
	(r#"x64/SensApi.lib"#, X64_SENSAPI),
	(r#"x64/SETUPAPI.lib"#, X64_SETUPAPI),
	(r#"x64/sfc.lib"#, X64_SFC),
	(r#"x64/SHDOCVW.lib"#, X64_SHDOCVW),
	(r#"x64/SHELL32.lib"#, X64_SHELL32),
	(r#"x64/SHLWAPI.lib"#, X64_SHLWAPI),
	(r#"x64/SLC.lib"#, X64_SLC),
	(r#"x64/slcext.lib"#, X64_SLCEXT),
	(r#"x64/SLWGA.lib"#, X64_SLWGA),
	(r#"x64/snmpapi.lib"#, X64_SNMPAPI),
	(r#"x64/srpapi.lib"#, X64_SRPAPI),
	(r#"x64/SspiCli.lib"#, X64_SSPICLI),
	(r#"x64/t2embed.lib"#, X64_T2EMBED),
	(r#"x64/TAPI32.lib"#, X64_TAPI32),
	(r#"x64/tbs.lib"#, X64_TBS),
	(r#"x64/TDH.lib"#, X64_TDH),
	(r#"x64/TOKENBINDING.lib"#, X64_TOKENBINDING),
	(r#"x64/TRAFFIC.lib"#, X64_TRAFFIC),
	(r#"x64/txfw32.lib"#, X64_TXFW32),
	(r#"x64/ualapi.lib"#, X64_UALAPI),
	(r#"x64/UIAutomationCore.lib"#, X64_UIAUTOMATIONCORE),
	(r#"x64/URLMON.lib"#, X64_URLMON),
	(r#"x64/USER32.lib"#, X64_USER32),
	(r#"x64/USERENV.lib"#, X64_USERENV),
	(r#"x64/USP10.lib"#, X64_USP10),
	(r#"x64/UXTHEME.lib"#, X64_UXTHEME),
	(r#"x64/verifier.lib"#, X64_VERIFIER),
	(r#"x64/VERSION.lib"#, X64_VERSION),
	(r#"x64/vertdll.lib"#, X64_VERTDLL),
	(r#"x64/VirtDisk.lib"#, X64_VIRTDISK),
	(r#"x64/VSSAPI.lib"#, X64_VSSAPI),
	(r#"x64/wcmapi.lib"#, X64_WCMAPI),
	(r#"x64/WDSBP.lib"#, X64_WDSBP),
	(r#"x64/WDSCLIENTAPI.lib"#, X64_WDSCLIENTAPI),
	(r#"x64/WDSMC.lib"#, X64_WDSMC),
	(r#"x64/WDSPXE.lib"#, X64_WDSPXE),
	(r#"x64/WDSTPTC.lib"#, X64_WDSTPTC),
	(r#"x64/webauthn.lib"#, X64_WEBAUTHN),
	(r#"x64/webservices.lib"#, X64_WEBSERVICES),
	(r#"x64/websocket.lib"#, X64_WEBSOCKET),
	(r#"x64/WecApi.lib"#, X64_WECAPI),
	(r#"x64/wer.lib"#, X64_WER),
	(r#"x64/wevtapi.lib"#, X64_WEVTAPI),
	(r#"x64/winbio.lib"#, X64_WINBIO),
	(r#"x64/windows.ai.machinelearning.lib"#, X64_WINDOWS_AI_MACHINELEARNING),
	(r#"x64/Windows.Data.Pdf.lib"#, X64_WINDOWS_DATA_PDF),
	(r#"x64/Windows.Networking.lib"#, X64_WINDOWS_NETWORKING),
	(r#"x64/Windows.UI.Xaml.lib"#, X64_WINDOWS_UI_XAML),
	(r#"x64/WindowsCodecs.lib"#, X64_WINDOWSCODECS),
	(r#"x64/WINFAX.lib"#, X64_WINFAX),
	(r#"x64/WINHTTP.lib"#, X64_WINHTTP),
	(r#"x64/WinHvEmulation.lib"#, X64_WINHVEMULATION),
	(r#"x64/WinHvPlatform.lib"#, X64_WINHVPLATFORM),
	(r#"x64/WININET.lib"#, X64_WININET),
	(r#"x64/winml.lib"#, X64_WINML),
	(r#"x64/WINMM.lib"#, X64_WINMM),
	(r#"x64/WinSCard.lib"#, X64_WINSCARD),
	(r#"x64/WINSPOOL.lib"#, X64_WINSPOOL),
	(r#"x64/WINTRUST.lib"#, X64_WINTRUST),
	(r#"x64/WINUSB.lib"#, X64_WINUSB),
	(r#"x64/wlanapi.lib"#, X64_WLANAPI),
	(r#"x64/wlanui.lib"#, X64_WLANUI),
	(r#"x64/WLDAP32.lib"#, X64_WLDAP32),
	(r#"x64/Wldp.lib"#, X64_WLDP),
	(r#"x64/WMVCore.lib"#, X64_WMVCORE),
	(r#"x64/wnvapi.lib"#, X64_WNVAPI),
	(r#"x64/WOFUTIL.lib"#, X64_WOFUTIL),
	(r#"x64/WS2_32.lib"#, X64_WS2_32),
	(r#"x64/WSCAPI.lib"#, X64_WSCAPI),
	(r#"x64/WSClient.lib"#, X64_WSCLIENT),
	(r#"x64/wsdapi.lib"#, X64_WSDAPI),
	(r#"x64/WsmSvc.lib"#, X64_WSMSVC),
	(r#"x64/wsnmp32.lib"#, X64_WSNMP32),
	(r#"x64/WTSAPI32.lib"#, X64_WTSAPI32),
	(r#"x64/XAudio2_8.lib"#, X64_XAUDIO2_8),
	(r#"x64/XINPUTUAP.lib"#, X64_XINPUTUAP),
	(r#"x64/XmlLite.lib"#, X64_XMLLITE),
	(r#"x86/ACLUI.lib"#, X86_ACLUI),
	(r#"x86/ACTIVEDS.lib"#, X86_ACTIVEDS),
	(r#"x86/ADVAPI32.lib"#, X86_ADVAPI32),
	(r#"x86/ADVPACK.lib"#, X86_ADVPACK),
	(r#"x86/Amsi.lib"#, X86_AMSI),
	(r#"x86/api-ms-win-appmodel-runtime-l1-1-1.lib"#, X86_API_MS_WIN_APPMODEL_RUNTIME_L1_1_1),
	(r#"x86/api-ms-win-appmodel-runtime-l1-1-3.lib"#, X86_API_MS_WIN_APPMODEL_RUNTIME_L1_1_3),
	(r#"x86/api-ms-win-core-apiquery-l2-1-0.lib"#, X86_API_MS_WIN_CORE_APIQUERY_L2_1_0),
	(r#"x86/api-ms-win-core-backgroundtask-l1-1-0.lib"#, X86_API_MS_WIN_CORE_BACKGROUNDTASK_L1_1_0),
	(r#"x86/api-ms-win-core-comm-l1-1-1.lib"#, X86_API_MS_WIN_CORE_COMM_L1_1_1),
	(r#"x86/api-ms-win-core-comm-l1-1-2.lib"#, X86_API_MS_WIN_CORE_COMM_L1_1_2),
	(r#"x86/api-ms-win-core-errorhandling-l1-1-3.lib"#, X86_API_MS_WIN_CORE_ERRORHANDLING_L1_1_3),
	(r#"x86/api-ms-win-core-featurestaging-l1-1-0.lib"#, X86_API_MS_WIN_CORE_FEATURESTAGING_L1_1_0),
	(r#"x86/api-ms-win-core-featurestaging-l1-1-1.lib"#, X86_API_MS_WIN_CORE_FEATURESTAGING_L1_1_1),
	(r#"x86/api-ms-win-core-file-fromapp-l1-1-0.lib"#, X86_API_MS_WIN_CORE_FILE_FROMAPP_L1_1_0),
	(r#"x86/api-ms-win-core-handle-l1-1-0.lib"#, X86_API_MS_WIN_CORE_HANDLE_L1_1_0),
	(r#"x86/api-ms-win-core-libraryloader-l2-1-0.lib"#, X86_API_MS_WIN_CORE_LIBRARYLOADER_L2_1_0),
	(r#"x86/api-ms-win-core-memory-l1-1-3.lib"#, X86_API_MS_WIN_CORE_MEMORY_L1_1_3),
	(r#"x86/api-ms-win-core-memory-l1-1-4.lib"#, X86_API_MS_WIN_CORE_MEMORY_L1_1_4),
	(r#"x86/api-ms-win-core-memory-l1-1-5.lib"#, X86_API_MS_WIN_CORE_MEMORY_L1_1_5),
	(r#"x86/api-ms-win-core-memory-l1-1-6.lib"#, X86_API_MS_WIN_CORE_MEMORY_L1_1_6),
	(r#"x86/api-ms-win-core-memory-l1-1-7.lib"#, X86_API_MS_WIN_CORE_MEMORY_L1_1_7),
	(r#"x86/api-ms-win-core-path-l1-1-0.lib"#, X86_API_MS_WIN_CORE_PATH_L1_1_0),
	(r#"x86/api-ms-win-core-psm-appnotify-l1-1-0.lib"#, X86_API_MS_WIN_CORE_PSM_APPNOTIFY_L1_1_0),
	(r#"x86/api-ms-win-core-psm-appnotify-l1-1-1.lib"#, X86_API_MS_WIN_CORE_PSM_APPNOTIFY_L1_1_1),
	(r#"x86/api-ms-win-core-realtime-l1-1-1.lib"#, X86_API_MS_WIN_CORE_REALTIME_L1_1_1),
	(r#"x86/api-ms-win-core-realtime-l1-1-2.lib"#, X86_API_MS_WIN_CORE_REALTIME_L1_1_2),
	(r#"x86/api-ms-win-core-slapi-l1-1-0.lib"#, X86_API_MS_WIN_CORE_SLAPI_L1_1_0),
	(r#"x86/api-ms-win-core-state-helpers-l1-1-0.lib"#, X86_API_MS_WIN_CORE_STATE_HELPERS_L1_1_0),
	(r#"x86/api-ms-win-core-sysinfo-l1-2-0.lib"#, X86_API_MS_WIN_CORE_SYSINFO_L1_2_0),
	(r#"x86/api-ms-win-core-sysinfo-l1-2-3.lib"#, X86_API_MS_WIN_CORE_SYSINFO_L1_2_3),
	(r#"x86/api-ms-win-core-sysinfo-l1-2-4.lib"#, X86_API_MS_WIN_CORE_SYSINFO_L1_2_4),
	(r#"x86/api-ms-win-core-util-l1-1-1.lib"#, X86_API_MS_WIN_CORE_UTIL_L1_1_1),
	(r#"x86/api-ms-win-core-winrt-error-l1-1-0.lib"#, X86_API_MS_WIN_CORE_WINRT_ERROR_L1_1_0),
	(r#"x86/api-ms-win-core-winrt-error-l1-1-1.lib"#, X86_API_MS_WIN_CORE_WINRT_ERROR_L1_1_1),
	(r#"x86/api-ms-win-core-winrt-l1-1-0.lib"#, X86_API_MS_WIN_CORE_WINRT_L1_1_0),
	(r#"x86/api-ms-win-core-winrt-registration-l1-1-0.lib"#, X86_API_MS_WIN_CORE_WINRT_REGISTRATION_L1_1_0),
	(r#"x86/api-ms-win-core-winrt-robuffer-l1-1-0.lib"#, X86_API_MS_WIN_CORE_WINRT_ROBUFFER_L1_1_0),
	(r#"x86/api-ms-win-core-winrt-roparameterizediid-l1-1-0.lib"#, X86_API_MS_WIN_CORE_WINRT_ROPARAMETERIZEDIID_L1_1_0),
	(r#"x86/api-ms-win-core-winrt-string-l1-1-0.lib"#, X86_API_MS_WIN_CORE_WINRT_STRING_L1_1_0),
	(r#"x86/api-ms-win-core-winrt-string-l1-1-1.lib"#, X86_API_MS_WIN_CORE_WINRT_STRING_L1_1_1),
	(r#"x86/api-ms-win-core-wow64-l1-1-1.lib"#, X86_API_MS_WIN_CORE_WOW64_L1_1_1),
	(r#"x86/api-ms-win-devices-query-l1-1-0.lib"#, X86_API_MS_WIN_DEVICES_QUERY_L1_1_0),
	(r#"x86/api-ms-win-devices-query-l1-1-1.lib"#, X86_API_MS_WIN_DEVICES_QUERY_L1_1_1),
	(r#"x86/api-ms-win-dx-d3dkmt-l1-1-0.lib"#, X86_API_MS_WIN_DX_D3DKMT_L1_1_0),
	(r#"x86/api-ms-win-gaming-deviceinformation-l1-1-0.lib"#, X86_API_MS_WIN_GAMING_DEVICEINFORMATION_L1_1_0),
	(r#"x86/api-ms-win-gaming-expandedresources-l1-1-0.lib"#, X86_API_MS_WIN_GAMING_EXPANDEDRESOURCES_L1_1_0),
	(r#"x86/api-ms-win-gaming-tcui-l1-1-0.lib"#, X86_API_MS_WIN_GAMING_TCUI_L1_1_0),
	(r#"x86/api-ms-win-gaming-tcui-l1-1-1.lib"#, X86_API_MS_WIN_GAMING_TCUI_L1_1_1),
	(r#"x86/api-ms-win-gaming-tcui-l1-1-2.lib"#, X86_API_MS_WIN_GAMING_TCUI_L1_1_2),
	(r#"x86/api-ms-win-gaming-tcui-l1-1-3.lib"#, X86_API_MS_WIN_GAMING_TCUI_L1_1_3),
	(r#"x86/api-ms-win-gaming-tcui-l1-1-4.lib"#, X86_API_MS_WIN_GAMING_TCUI_L1_1_4),
	(r#"x86/api-ms-win-mm-misc-l1-1-1.lib"#, X86_API_MS_WIN_MM_MISC_L1_1_1),
	(r#"x86/api-ms-win-net-isolation-l1-1-0.lib"#, X86_API_MS_WIN_NET_ISOLATION_L1_1_0),
	(r#"x86/api-ms-win-security-base-l1-2-2.lib"#, X86_API_MS_WIN_SECURITY_BASE_L1_2_2),
	(r#"x86/api-ms-win-security-isolatedcontainer-l1-1-0.lib"#, X86_API_MS_WIN_SECURITY_ISOLATEDCONTAINER_L1_1_0),
	(r#"x86/api-ms-win-security-isolatedcontainer-l1-1-1.lib"#, X86_API_MS_WIN_SECURITY_ISOLATEDCONTAINER_L1_1_1),
	(r#"x86/api-ms-win-service-core-l1-1-3.lib"#, X86_API_MS_WIN_SERVICE_CORE_L1_1_3),
	(r#"x86/api-ms-win-service-core-l1-1-4.lib"#, X86_API_MS_WIN_SERVICE_CORE_L1_1_4),
	(r#"x86/api-ms-win-shcore-scaling-l1-1-0.lib"#, X86_API_MS_WIN_SHCORE_SCALING_L1_1_0),
	(r#"x86/api-ms-win-shcore-scaling-l1-1-1.lib"#, X86_API_MS_WIN_SHCORE_SCALING_L1_1_1),
	(r#"x86/api-ms-win-shcore-scaling-l1-1-2.lib"#, X86_API_MS_WIN_SHCORE_SCALING_L1_1_2),
	(r#"x86/api-ms-win-shcore-stream-winrt-l1-1-0.lib"#, X86_API_MS_WIN_SHCORE_STREAM_WINRT_L1_1_0),
	(r#"x86/APPHELP.lib"#, X86_APPHELP),
	(r#"x86/AUTHZ.lib"#, X86_AUTHZ),
	(r#"x86/AVICAP32.lib"#, X86_AVICAP32),
	(r#"x86/AVIFIL32.lib"#, X86_AVIFIL32),
	(r#"x86/bcrypt.lib"#, X86_BCRYPT),
	(r#"x86/BluetoothApis.lib"#, X86_BLUETOOTHAPIS),
	(r#"x86/bthprops.lib"#, X86_BTHPROPS),
	(r#"x86/Cabinet.lib"#, X86_CABINET),
	(r#"x86/certadm.lib"#, X86_CERTADM),
	(r#"x86/certpoleng.lib"#, X86_CERTPOLENG),
	(r#"x86/CFGMGR32.lib"#, X86_CFGMGR32),
	(r#"x86/chakra.lib"#, X86_CHAKRA),
	(r#"x86/cldapi.lib"#, X86_CLDAPI),
	(r#"x86/clfsw32.lib"#, X86_CLFSW32),
	(r#"x86/CLUSAPI.lib"#, X86_CLUSAPI),
	(r#"x86/COMCTL32.lib"#, X86_COMCTL32),
	(r#"x86/COMDLG32.lib"#, X86_COMDLG32),
	(r#"x86/comsvcs.lib"#, X86_COMSVCS),
	(r#"x86/CoreMessaging.lib"#, X86_COREMESSAGING),
	(r#"x86/credui.lib"#, X86_CREDUI),
	(r#"x86/CRYPT32.lib"#, X86_CRYPT32),
	(r#"x86/CRYPTNET.lib"#, X86_CRYPTNET),
	(r#"x86/CRYPTUI.lib"#, X86_CRYPTUI),
	(r#"x86/CRYPTXML.lib"#, X86_CRYPTXML),
	(r#"x86/CSCAPI.lib"#, X86_CSCAPI),
	(r#"x86/d2d1.lib"#, X86_D2D1),
	(r#"x86/d3d10.lib"#, X86_D3D10),
	(r#"x86/d3d10_1.lib"#, X86_D3D10_1),
	(r#"x86/d3d11.lib"#, X86_D3D11),
	(r#"x86/d3d12.lib"#, X86_D3D12),
	(r#"x86/d3d9.lib"#, X86_D3D9),
	(r#"x86/D3DCOMPILER_47.lib"#, X86_D3DCOMPILER_47),
	(r#"x86/d3dcsx.lib"#, X86_D3DCSX),
	(r#"x86/davclnt.lib"#, X86_DAVCLNT),
	(r#"x86/dbgeng.lib"#, X86_DBGENG),
	(r#"x86/dbghelp.lib"#, X86_DBGHELP),
	(r#"x86/dbgmodel.lib"#, X86_DBGMODEL),
	(r#"x86/DCIMAN32.lib"#, X86_DCIMAN32),
	(r#"x86/dcomp.lib"#, X86_DCOMP),
	(r#"x86/DDRAW.lib"#, X86_DDRAW),
	(r#"x86/deviceaccess.lib"#, X86_DEVICEACCESS),
	(r#"x86/dflayout.lib"#, X86_DFLAYOUT),
	(r#"x86/dhcpcsvc.lib"#, X86_DHCPCSVC),
	(r#"x86/dhcpcsvc6.lib"#, X86_DHCPCSVC6),
	(r#"x86/DHCPSAPI.lib"#, X86_DHCPSAPI),
	(r#"x86/DiagnosticDataQuery.lib"#, X86_DIAGNOSTICDATAQUERY),
	(r#"x86/DINPUT8.lib"#, X86_DINPUT8),
	(r#"x86/DirectML.lib"#, X86_DIRECTML),
	(r#"x86/DNSAPI.lib"#, X86_DNSAPI),
	(r#"x86/drt.lib"#, X86_DRT),
	(r#"x86/drtprov.lib"#, X86_DRTPROV),
	(r#"x86/drttransport.lib"#, X86_DRTTRANSPORT),
	(r#"x86/DSOUND.lib"#, X86_DSOUND),
	(r#"x86/DSPARSE.lib"#, X86_DSPARSE),
	(r#"x86/dsprop.lib"#, X86_DSPROP),
	(r#"x86/DSSEC.lib"#, X86_DSSEC),
	(r#"x86/dsuiext.lib"#, X86_DSUIEXT),
	(r#"x86/dwmapi.lib"#, X86_DWMAPI),
	(r#"x86/DWrite.lib"#, X86_DWRITE),
	(r#"x86/dxgi.lib"#, X86_DXGI),
	(r#"x86/dxva2.lib"#, X86_DXVA2),
	(r#"x86/eappcfg.lib"#, X86_EAPPCFG),
	(r#"x86/eappprxy.lib"#, X86_EAPPPRXY),
	(r#"x86/efswrt.lib"#, X86_EFSWRT),
	(r#"x86/elscore.lib"#, X86_ELSCORE),
	(r#"x86/ESENT.lib"#, X86_ESENT),
	(r#"x86/EVR.lib"#, X86_EVR),
	(r#"x86/faultrep.lib"#, X86_FAULTREP),
	(r#"x86/fhsvcctl.lib"#, X86_FHSVCCTL),
	(r#"x86/FLTLIB.lib"#, X86_FLTLIB),
	(r#"x86/FONTSUB.lib"#, X86_FONTSUB),
	(r#"x86/fwpuclnt.lib"#, X86_FWPUCLNT),
	(r#"x86/fxsutility.lib"#, X86_FXSUTILITY),
	(r#"x86/GDI32.lib"#, X86_GDI32),
	(r#"x86/GPEDIT.lib"#, X86_GPEDIT),
	(r#"x86/HID.lib"#, X86_HID),
	(r#"x86/hlink.lib"#, X86_HLINK),
	(r#"x86/HrtfApo.lib"#, X86_HRTFAPO),
	(r#"x86/HTTPAPI.lib"#, X86_HTTPAPI),
	(r#"x86/ICM32.lib"#, X86_ICM32),
	(r#"x86/ICMUI.lib"#, X86_ICMUI),
	(r#"x86/icu.lib"#, X86_ICU),
	(r#"x86/IMM32.lib"#, X86_IMM32),
	(r#"x86/inkobjcore.lib"#, X86_INKOBJCORE),
	(r#"x86/IPHLPAPI.lib"#, X86_IPHLPAPI),
	(r#"x86/ISCSIDSC.lib"#, X86_ISCSIDSC),
	(r#"x86/KERNEL32.lib"#, X86_KERNEL32),
	(r#"x86/KeyCredMgr.lib"#, X86_KEYCREDMGR),
	(r#"x86/ksuser.lib"#, X86_KSUSER),
	(r#"x86/ktmw32.lib"#, X86_KTMW32),
	(r#"x86/loadperf.lib"#, X86_LOADPERF),
	(r#"x86/MAGNIFICATION.lib"#, X86_MAGNIFICATION),
	(r#"x86/MAPI32.lib"#, X86_MAPI32),
	(r#"x86/MDMRegistration.lib"#, X86_MDMREGISTRATION),
	(r#"x86/MF.lib"#, X86_MF),
	(r#"x86/MFCORE.lib"#, X86_MFCORE),
	(r#"x86/MFPlat.lib"#, X86_MFPLAT),
	(r#"x86/MFPlay.lib"#, X86_MFPLAY),
	(r#"x86/MFReadWrite.lib"#, X86_MFREADWRITE),
	(r#"x86/MFSENSORGROUP.lib"#, X86_MFSENSORGROUP),
	(r#"x86/mfsrcsnk.lib"#, X86_MFSRCSNK),
	(r#"x86/mgmtapi.lib"#, X86_MGMTAPI),
	(r#"x86/mi.lib"#, X86_MI),
	(r#"x86/MMDevAPI.lib"#, X86_MMDEVAPI),
	(r#"x86/MPR.lib"#, X86_MPR),
	(r#"x86/MPRAPI.lib"#, X86_MPRAPI),
	(r#"x86/MrmSupport.lib"#, X86_MRMSUPPORT),
	(r#"x86/MSACM32.lib"#, X86_MSACM32),
	(r#"x86/MSAJApi.lib"#, X86_MSAJAPI),
	(r#"x86/mscms.lib"#, X86_MSCMS),
	(r#"x86/MsCtfMonitor.lib"#, X86_MSCTFMONITOR),
	(r#"x86/msdmo.lib"#, X86_MSDMO),
	(r#"x86/msdrm.lib"#, X86_MSDRM),
	(r#"x86/msi.lib"#, X86_MSI),
	(r#"x86/MSIMG32.lib"#, X86_MSIMG32),
	(r#"x86/MSPORTS.lib"#, X86_MSPORTS),
	(r#"x86/mstask.lib"#, X86_MSTASK),
	(r#"x86/MSVFW32.lib"#, X86_MSVFW32),
	(r#"x86/MSWSOCK.lib"#, X86_MSWSOCK),
	(r#"x86/MTxDM.lib"#, X86_MTXDM),
	(r#"x86/ncrypt.lib"#, X86_NCRYPT),
	(r#"x86/NDFAPI.lib"#, X86_NDFAPI),
	(r#"x86/NETAPI32.lib"#, X86_NETAPI32),
	(r#"x86/NETSH.lib"#, X86_NETSH),
	(r#"x86/newdev.lib"#, X86_NEWDEV),
	(r#"x86/NInput.lib"#, X86_NINPUT),
	(r#"x86/NORMALIZ.lib"#, X86_NORMALIZ),
	(r#"x86/ntdll.lib"#, X86_NTDLL),
	(r#"x86/NTDSAPI.lib"#, X86_NTDSAPI),
	(r#"x86/NTLANMAN.lib"#, X86_NTLANMAN),
	(r#"x86/OLE32.lib"#, X86_OLE32),
	(r#"x86/OLEACC.lib"#, X86_OLEACC),
	(r#"x86/OLEAUT32.lib"#, X86_OLEAUT32),
	(r#"x86/oledlg.lib"#, X86_OLEDLG),
	(r#"x86/OnDemandConnRouteHelper.lib"#, X86_ONDEMANDCONNROUTEHELPER),
	(r#"x86/OPENGL32.lib"#, X86_OPENGL32),
	(r#"x86/P2P.lib"#, X86_P2P),
	(r#"x86/P2PGRAPH.lib"#, X86_P2PGRAPH),
	(r#"x86/pdh.lib"#, X86_PDH),
	(r#"x86/PeerDist.lib"#, X86_PEERDIST),
	(r#"x86/POWRPROF.lib"#, X86_POWRPROF),
	(r#"x86/prntvpt.lib"#, X86_PRNTVPT),
	(r#"x86/PROJECTEDFSLIB.lib"#, X86_PROJECTEDFSLIB),
	(r#"x86/PROPSYS.lib"#, X86_PROPSYS),
	(r#"x86/QUARTZ.lib"#, X86_QUARTZ),
	(r#"x86/query.lib"#, X86_QUERY),
	(r#"x86/qwave.lib"#, X86_QWAVE),
	(r#"x86/RASAPI32.lib"#, X86_RASAPI32),
	(r#"x86/RASDLG.lib"#, X86_RASDLG),
	(r#"x86/RESUTILS.lib"#, X86_RESUTILS),
	(r#"x86/RoMetadata.lib"#, X86_ROMETADATA),
	(r#"x86/RPCNS4.lib"#, X86_RPCNS4),
	(r#"x86/RPCRT4.lib"#, X86_RPCRT4),
	(r#"x86/RstrtMgr.lib"#, X86_RSTRTMGR),
	(r#"x86/rtm.lib"#, X86_RTM),
	(r#"x86/SAS.lib"#, X86_SAS),
	(r#"x86/SCARDDLG.lib"#, X86_SCARDDLG),
	(r#"x86/SCHANNEL.lib"#, X86_SCHANNEL),
	(r#"x86/SECUR32.lib"#, X86_SECUR32),
	(r#"x86/SensApi.lib"#, X86_SENSAPI),
	(r#"x86/SETUPAPI.lib"#, X86_SETUPAPI),
	(r#"x86/sfc.lib"#, X86_SFC),
	(r#"x86/SHDOCVW.lib"#, X86_SHDOCVW),
	(r#"x86/SHELL32.lib"#, X86_SHELL32),
	(r#"x86/SHLWAPI.lib"#, X86_SHLWAPI),
	(r#"x86/SLC.lib"#, X86_SLC),
	(r#"x86/slcext.lib"#, X86_SLCEXT),
	(r#"x86/SLWGA.lib"#, X86_SLWGA),
	(r#"x86/snmpapi.lib"#, X86_SNMPAPI),
	(r#"x86/srpapi.lib"#, X86_SRPAPI),
	(r#"x86/SspiCli.lib"#, X86_SSPICLI),
	(r#"x86/t2embed.lib"#, X86_T2EMBED),
	(r#"x86/TAPI32.lib"#, X86_TAPI32),
	(r#"x86/tbs.lib"#, X86_TBS),
	(r#"x86/TDH.lib"#, X86_TDH),
	(r#"x86/TOKENBINDING.lib"#, X86_TOKENBINDING),
	(r#"x86/TRAFFIC.lib"#, X86_TRAFFIC),
	(r#"x86/txfw32.lib"#, X86_TXFW32),
	(r#"x86/ualapi.lib"#, X86_UALAPI),
	(r#"x86/UIAutomationCore.lib"#, X86_UIAUTOMATIONCORE),
	(r#"x86/URLMON.lib"#, X86_URLMON),
	(r#"x86/USER32.lib"#, X86_USER32),
	(r#"x86/USERENV.lib"#, X86_USERENV),
	(r#"x86/USP10.lib"#, X86_USP10),
	(r#"x86/UXTHEME.lib"#, X86_UXTHEME),
	(r#"x86/verifier.lib"#, X86_VERIFIER),
	(r#"x86/VERSION.lib"#, X86_VERSION),
	(r#"x86/vertdll.lib"#, X86_VERTDLL),
	(r#"x86/VirtDisk.lib"#, X86_VIRTDISK),
	(r#"x86/VSSAPI.lib"#, X86_VSSAPI),
	(r#"x86/wcmapi.lib"#, X86_WCMAPI),
	(r#"x86/WDSBP.lib"#, X86_WDSBP),
	(r#"x86/WDSCLIENTAPI.lib"#, X86_WDSCLIENTAPI),
	(r#"x86/WDSMC.lib"#, X86_WDSMC),
	(r#"x86/WDSPXE.lib"#, X86_WDSPXE),
	(r#"x86/WDSTPTC.lib"#, X86_WDSTPTC),
	(r#"x86/webauthn.lib"#, X86_WEBAUTHN),
	(r#"x86/webservices.lib"#, X86_WEBSERVICES),
	(r#"x86/websocket.lib"#, X86_WEBSOCKET),
	(r#"x86/WecApi.lib"#, X86_WECAPI),
	(r#"x86/wer.lib"#, X86_WER),
	(r#"x86/wevtapi.lib"#, X86_WEVTAPI),
	(r#"x86/winbio.lib"#, X86_WINBIO),
	(r#"x86/windows.ai.machinelearning.lib"#, X86_WINDOWS_AI_MACHINELEARNING),
	(r#"x86/Windows.Data.Pdf.lib"#, X86_WINDOWS_DATA_PDF),
	(r#"x86/Windows.Networking.lib"#, X86_WINDOWS_NETWORKING),
	(r#"x86/Windows.UI.Xaml.lib"#, X86_WINDOWS_UI_XAML),
	(r#"x86/WindowsCodecs.lib"#, X86_WINDOWSCODECS),
	(r#"x86/WINFAX.lib"#, X86_WINFAX),
	(r#"x86/WINHTTP.lib"#, X86_WINHTTP),
	(r#"x86/WinHvEmulation.lib"#, X86_WINHVEMULATION),
	(r#"x86/WinHvPlatform.lib"#, X86_WINHVPLATFORM),
	(r#"x86/WININET.lib"#, X86_WININET),
	(r#"x86/winml.lib"#, X86_WINML),
	(r#"x86/WINMM.lib"#, X86_WINMM),
	(r#"x86/WinSCard.lib"#, X86_WINSCARD),
	(r#"x86/WINSPOOL.lib"#, X86_WINSPOOL),
	(r#"x86/WINTRUST.lib"#, X86_WINTRUST),
	(r#"x86/WINUSB.lib"#, X86_WINUSB),
	(r#"x86/wlanapi.lib"#, X86_WLANAPI),
	(r#"x86/wlanui.lib"#, X86_WLANUI),
	(r#"x86/WLDAP32.lib"#, X86_WLDAP32),
	(r#"x86/Wldp.lib"#, X86_WLDP),
	(r#"x86/WMVCore.lib"#, X86_WMVCORE),
	(r#"x86/wnvapi.lib"#, X86_WNVAPI),
	(r#"x86/WOFUTIL.lib"#, X86_WOFUTIL),
	(r#"x86/WS2_32.lib"#, X86_WS2_32),
	(r#"x86/WSCAPI.lib"#, X86_WSCAPI),
	(r#"x86/WSClient.lib"#, X86_WSCLIENT),
	(r#"x86/wsdapi.lib"#, X86_WSDAPI),
	(r#"x86/WsmSvc.lib"#, X86_WSMSVC),
	(r#"x86/wsnmp32.lib"#, X86_WSNMP32),
	(r#"x86/WTSAPI32.lib"#, X86_WTSAPI32),
	(r#"x86/XAudio2_8.lib"#, X86_XAUDIO2_8),
	(r#"x86/XINPUTUAP.lib"#, X86_XINPUTUAP),
	(r#"x86/XmlLite.lib"#, X86_XMLLITE),
	(r#"arm/ACLUI.lib"#, ARM_ACLUI),
	(r#"arm/ACTIVEDS.lib"#, ARM_ACTIVEDS),
	(r#"arm/ADVAPI32.lib"#, ARM_ADVAPI32),
	(r#"arm/ADVPACK.lib"#, ARM_ADVPACK),
	(r#"arm/Amsi.lib"#, ARM_AMSI),
	(r#"arm/api-ms-win-appmodel-runtime-l1-1-1.lib"#, ARM_API_MS_WIN_APPMODEL_RUNTIME_L1_1_1),
	(r#"arm/api-ms-win-appmodel-runtime-l1-1-3.lib"#, ARM_API_MS_WIN_APPMODEL_RUNTIME_L1_1_3),
	(r#"arm/api-ms-win-core-apiquery-l2-1-0.lib"#, ARM_API_MS_WIN_CORE_APIQUERY_L2_1_0),
	(r#"arm/api-ms-win-core-backgroundtask-l1-1-0.lib"#, ARM_API_MS_WIN_CORE_BACKGROUNDTASK_L1_1_0),
	(r#"arm/api-ms-win-core-comm-l1-1-1.lib"#, ARM_API_MS_WIN_CORE_COMM_L1_1_1),
	(r#"arm/api-ms-win-core-comm-l1-1-2.lib"#, ARM_API_MS_WIN_CORE_COMM_L1_1_2),
	(r#"arm/api-ms-win-core-errorhandling-l1-1-3.lib"#, ARM_API_MS_WIN_CORE_ERRORHANDLING_L1_1_3),
	(r#"arm/api-ms-win-core-featurestaging-l1-1-0.lib"#, ARM_API_MS_WIN_CORE_FEATURESTAGING_L1_1_0),
	(r#"arm/api-ms-win-core-featurestaging-l1-1-1.lib"#, ARM_API_MS_WIN_CORE_FEATURESTAGING_L1_1_1),
	(r#"arm/api-ms-win-core-file-fromapp-l1-1-0.lib"#, ARM_API_MS_WIN_CORE_FILE_FROMAPP_L1_1_0),
	(r#"arm/api-ms-win-core-handle-l1-1-0.lib"#, ARM_API_MS_WIN_CORE_HANDLE_L1_1_0),
	(r#"arm/api-ms-win-core-libraryloader-l2-1-0.lib"#, ARM_API_MS_WIN_CORE_LIBRARYLOADER_L2_1_0),
	(r#"arm/api-ms-win-core-memory-l1-1-3.lib"#, ARM_API_MS_WIN_CORE_MEMORY_L1_1_3),
	(r#"arm/api-ms-win-core-memory-l1-1-4.lib"#, ARM_API_MS_WIN_CORE_MEMORY_L1_1_4),
	(r#"arm/api-ms-win-core-memory-l1-1-5.lib"#, ARM_API_MS_WIN_CORE_MEMORY_L1_1_5),
	(r#"arm/api-ms-win-core-memory-l1-1-6.lib"#, ARM_API_MS_WIN_CORE_MEMORY_L1_1_6),
	(r#"arm/api-ms-win-core-memory-l1-1-7.lib"#, ARM_API_MS_WIN_CORE_MEMORY_L1_1_7),
	(r#"arm/api-ms-win-core-path-l1-1-0.lib"#, ARM_API_MS_WIN_CORE_PATH_L1_1_0),
	(r#"arm/api-ms-win-core-psm-appnotify-l1-1-0.lib"#, ARM_API_MS_WIN_CORE_PSM_APPNOTIFY_L1_1_0),
	(r#"arm/api-ms-win-core-psm-appnotify-l1-1-1.lib"#, ARM_API_MS_WIN_CORE_PSM_APPNOTIFY_L1_1_1),
	(r#"arm/api-ms-win-core-realtime-l1-1-1.lib"#, ARM_API_MS_WIN_CORE_REALTIME_L1_1_1),
	(r#"arm/api-ms-win-core-realtime-l1-1-2.lib"#, ARM_API_MS_WIN_CORE_REALTIME_L1_1_2),
	(r#"arm/api-ms-win-core-slapi-l1-1-0.lib"#, ARM_API_MS_WIN_CORE_SLAPI_L1_1_0),
	(r#"arm/api-ms-win-core-state-helpers-l1-1-0.lib"#, ARM_API_MS_WIN_CORE_STATE_HELPERS_L1_1_0),
	(r#"arm/api-ms-win-core-sysinfo-l1-2-0.lib"#, ARM_API_MS_WIN_CORE_SYSINFO_L1_2_0),
	(r#"arm/api-ms-win-core-sysinfo-l1-2-3.lib"#, ARM_API_MS_WIN_CORE_SYSINFO_L1_2_3),
	(r#"arm/api-ms-win-core-sysinfo-l1-2-4.lib"#, ARM_API_MS_WIN_CORE_SYSINFO_L1_2_4),
	(r#"arm/api-ms-win-core-util-l1-1-1.lib"#, ARM_API_MS_WIN_CORE_UTIL_L1_1_1),
	(r#"arm/api-ms-win-core-winrt-error-l1-1-0.lib"#, ARM_API_MS_WIN_CORE_WINRT_ERROR_L1_1_0),
	(r#"arm/api-ms-win-core-winrt-error-l1-1-1.lib"#, ARM_API_MS_WIN_CORE_WINRT_ERROR_L1_1_1),
	(r#"arm/api-ms-win-core-winrt-l1-1-0.lib"#, ARM_API_MS_WIN_CORE_WINRT_L1_1_0),
	(r#"arm/api-ms-win-core-winrt-registration-l1-1-0.lib"#, ARM_API_MS_WIN_CORE_WINRT_REGISTRATION_L1_1_0),
	(r#"arm/api-ms-win-core-winrt-robuffer-l1-1-0.lib"#, ARM_API_MS_WIN_CORE_WINRT_ROBUFFER_L1_1_0),
	(r#"arm/api-ms-win-core-winrt-roparameterizediid-l1-1-0.lib"#, ARM_API_MS_WIN_CORE_WINRT_ROPARAMETERIZEDIID_L1_1_0),
	(r#"arm/api-ms-win-core-winrt-string-l1-1-0.lib"#, ARM_API_MS_WIN_CORE_WINRT_STRING_L1_1_0),
	(r#"arm/api-ms-win-core-winrt-string-l1-1-1.lib"#, ARM_API_MS_WIN_CORE_WINRT_STRING_L1_1_1),
	(r#"arm/api-ms-win-core-wow64-l1-1-1.lib"#, ARM_API_MS_WIN_CORE_WOW64_L1_1_1),
	(r#"arm/api-ms-win-devices-query-l1-1-0.lib"#, ARM_API_MS_WIN_DEVICES_QUERY_L1_1_0),
	(r#"arm/api-ms-win-devices-query-l1-1-1.lib"#, ARM_API_MS_WIN_DEVICES_QUERY_L1_1_1),
	(r#"arm/api-ms-win-dx-d3dkmt-l1-1-0.lib"#, ARM_API_MS_WIN_DX_D3DKMT_L1_1_0),
	(r#"arm/api-ms-win-gaming-deviceinformation-l1-1-0.lib"#, ARM_API_MS_WIN_GAMING_DEVICEINFORMATION_L1_1_0),
	(r#"arm/api-ms-win-gaming-expandedresources-l1-1-0.lib"#, ARM_API_MS_WIN_GAMING_EXPANDEDRESOURCES_L1_1_0),
	(r#"arm/api-ms-win-gaming-tcui-l1-1-0.lib"#, ARM_API_MS_WIN_GAMING_TCUI_L1_1_0),
	(r#"arm/api-ms-win-gaming-tcui-l1-1-1.lib"#, ARM_API_MS_WIN_GAMING_TCUI_L1_1_1),
	(r#"arm/api-ms-win-gaming-tcui-l1-1-2.lib"#, ARM_API_MS_WIN_GAMING_TCUI_L1_1_2),
	(r#"arm/api-ms-win-gaming-tcui-l1-1-3.lib"#, ARM_API_MS_WIN_GAMING_TCUI_L1_1_3),
	(r#"arm/api-ms-win-gaming-tcui-l1-1-4.lib"#, ARM_API_MS_WIN_GAMING_TCUI_L1_1_4),
	(r#"arm/api-ms-win-mm-misc-l1-1-1.lib"#, ARM_API_MS_WIN_MM_MISC_L1_1_1),
	(r#"arm/api-ms-win-net-isolation-l1-1-0.lib"#, ARM_API_MS_WIN_NET_ISOLATION_L1_1_0),
	(r#"arm/api-ms-win-security-base-l1-2-2.lib"#, ARM_API_MS_WIN_SECURITY_BASE_L1_2_2),
	(r#"arm/api-ms-win-security-isolatedcontainer-l1-1-0.lib"#, ARM_API_MS_WIN_SECURITY_ISOLATEDCONTAINER_L1_1_0),
	(r#"arm/api-ms-win-security-isolatedcontainer-l1-1-1.lib"#, ARM_API_MS_WIN_SECURITY_ISOLATEDCONTAINER_L1_1_1),
	(r#"arm/api-ms-win-service-core-l1-1-3.lib"#, ARM_API_MS_WIN_SERVICE_CORE_L1_1_3),
	(r#"arm/api-ms-win-service-core-l1-1-4.lib"#, ARM_API_MS_WIN_SERVICE_CORE_L1_1_4),
	(r#"arm/api-ms-win-shcore-scaling-l1-1-0.lib"#, ARM_API_MS_WIN_SHCORE_SCALING_L1_1_0),
	(r#"arm/api-ms-win-shcore-scaling-l1-1-1.lib"#, ARM_API_MS_WIN_SHCORE_SCALING_L1_1_1),
	(r#"arm/api-ms-win-shcore-scaling-l1-1-2.lib"#, ARM_API_MS_WIN_SHCORE_SCALING_L1_1_2),
	(r#"arm/api-ms-win-shcore-stream-winrt-l1-1-0.lib"#, ARM_API_MS_WIN_SHCORE_STREAM_WINRT_L1_1_0),
	(r#"arm/APPHELP.lib"#, ARM_APPHELP),
	(r#"arm/AUTHZ.lib"#, ARM_AUTHZ),
	(r#"arm/AVICAP32.lib"#, ARM_AVICAP32),
	(r#"arm/AVIFIL32.lib"#, ARM_AVIFIL32),
	(r#"arm/bcrypt.lib"#, ARM_BCRYPT),
	(r#"arm/BluetoothApis.lib"#, ARM_BLUETOOTHAPIS),
	(r#"arm/bthprops.lib"#, ARM_BTHPROPS),
	(r#"arm/Cabinet.lib"#, ARM_CABINET),
	(r#"arm/certadm.lib"#, ARM_CERTADM),
	(r#"arm/certpoleng.lib"#, ARM_CERTPOLENG),
	(r#"arm/CFGMGR32.lib"#, ARM_CFGMGR32),
	(r#"arm/chakra.lib"#, ARM_CHAKRA),
	(r#"arm/cldapi.lib"#, ARM_CLDAPI),
	(r#"arm/clfsw32.lib"#, ARM_CLFSW32),
	(r#"arm/CLUSAPI.lib"#, ARM_CLUSAPI),
	(r#"arm/COMCTL32.lib"#, ARM_COMCTL32),
	(r#"arm/COMDLG32.lib"#, ARM_COMDLG32),
	(r#"arm/comsvcs.lib"#, ARM_COMSVCS),
	(r#"arm/CoreMessaging.lib"#, ARM_COREMESSAGING),
	(r#"arm/credui.lib"#, ARM_CREDUI),
	(r#"arm/CRYPT32.lib"#, ARM_CRYPT32),
	(r#"arm/CRYPTNET.lib"#, ARM_CRYPTNET),
	(r#"arm/CRYPTUI.lib"#, ARM_CRYPTUI),
	(r#"arm/CRYPTXML.lib"#, ARM_CRYPTXML),
	(r#"arm/CSCAPI.lib"#, ARM_CSCAPI),
	(r#"arm/d2d1.lib"#, ARM_D2D1),
	(r#"arm/d3d10.lib"#, ARM_D3D10),
	(r#"arm/d3d10_1.lib"#, ARM_D3D10_1),
	(r#"arm/d3d11.lib"#, ARM_D3D11),
	(r#"arm/d3d12.lib"#, ARM_D3D12),
	(r#"arm/d3d9.lib"#, ARM_D3D9),
	(r#"arm/D3DCOMPILER_47.lib"#, ARM_D3DCOMPILER_47),
	(r#"arm/d3dcsx.lib"#, ARM_D3DCSX),
	(r#"arm/davclnt.lib"#, ARM_DAVCLNT),
	(r#"arm/dbgeng.lib"#, ARM_DBGENG),
	(r#"arm/dbghelp.lib"#, ARM_DBGHELP),
	(r#"arm/dbgmodel.lib"#, ARM_DBGMODEL),
	(r#"arm/DCIMAN32.lib"#, ARM_DCIMAN32),
	(r#"arm/dcomp.lib"#, ARM_DCOMP),
	(r#"arm/DDRAW.lib"#, ARM_DDRAW),
	(r#"arm/deviceaccess.lib"#, ARM_DEVICEACCESS),
	(r#"arm/dflayout.lib"#, ARM_DFLAYOUT),
	(r#"arm/dhcpcsvc.lib"#, ARM_DHCPCSVC),
	(r#"arm/dhcpcsvc6.lib"#, ARM_DHCPCSVC6),
	(r#"arm/DHCPSAPI.lib"#, ARM_DHCPSAPI),
	(r#"arm/DiagnosticDataQuery.lib"#, ARM_DIAGNOSTICDATAQUERY),
	(r#"arm/DINPUT8.lib"#, ARM_DINPUT8),
	(r#"arm/DirectML.lib"#, ARM_DIRECTML),
	(r#"arm/DNSAPI.lib"#, ARM_DNSAPI),
	(r#"arm/drt.lib"#, ARM_DRT),
	(r#"arm/drtprov.lib"#, ARM_DRTPROV),
	(r#"arm/drttransport.lib"#, ARM_DRTTRANSPORT),
	(r#"arm/DSOUND.lib"#, ARM_DSOUND),
	(r#"arm/DSPARSE.lib"#, ARM_DSPARSE),
	(r#"arm/dsprop.lib"#, ARM_DSPROP),
	(r#"arm/DSSEC.lib"#, ARM_DSSEC),
	(r#"arm/dsuiext.lib"#, ARM_DSUIEXT),
	(r#"arm/dwmapi.lib"#, ARM_DWMAPI),
	(r#"arm/DWrite.lib"#, ARM_DWRITE),
	(r#"arm/dxgi.lib"#, ARM_DXGI),
	(r#"arm/dxva2.lib"#, ARM_DXVA2),
	(r#"arm/eappcfg.lib"#, ARM_EAPPCFG),
	(r#"arm/eappprxy.lib"#, ARM_EAPPPRXY),
	(r#"arm/efswrt.lib"#, ARM_EFSWRT),
	(r#"arm/elscore.lib"#, ARM_ELSCORE),
	(r#"arm/ESENT.lib"#, ARM_ESENT),
	(r#"arm/EVR.lib"#, ARM_EVR),
	(r#"arm/faultrep.lib"#, ARM_FAULTREP),
	(r#"arm/fhsvcctl.lib"#, ARM_FHSVCCTL),
	(r#"arm/FLTLIB.lib"#, ARM_FLTLIB),
	(r#"arm/FONTSUB.lib"#, ARM_FONTSUB),
	(r#"arm/fwpuclnt.lib"#, ARM_FWPUCLNT),
	(r#"arm/fxsutility.lib"#, ARM_FXSUTILITY),
	(r#"arm/GDI32.lib"#, ARM_GDI32),
	(r#"arm/GPEDIT.lib"#, ARM_GPEDIT),
	(r#"arm/HID.lib"#, ARM_HID),
	(r#"arm/hlink.lib"#, ARM_HLINK),
	(r#"arm/HrtfApo.lib"#, ARM_HRTFAPO),
	(r#"arm/HTTPAPI.lib"#, ARM_HTTPAPI),
	(r#"arm/ICM32.lib"#, ARM_ICM32),
	(r#"arm/ICMUI.lib"#, ARM_ICMUI),
	(r#"arm/icu.lib"#, ARM_ICU),
	(r#"arm/IMM32.lib"#, ARM_IMM32),
	(r#"arm/inkobjcore.lib"#, ARM_INKOBJCORE),
	(r#"arm/IPHLPAPI.lib"#, ARM_IPHLPAPI),
	(r#"arm/ISCSIDSC.lib"#, ARM_ISCSIDSC),
	(r#"arm/KERNEL32.lib"#, ARM_KERNEL32),
	(r#"arm/KeyCredMgr.lib"#, ARM_KEYCREDMGR),
	(r#"arm/ksuser.lib"#, ARM_KSUSER),
	(r#"arm/ktmw32.lib"#, ARM_KTMW32),
	(r#"arm/loadperf.lib"#, ARM_LOADPERF),
	(r#"arm/MAGNIFICATION.lib"#, ARM_MAGNIFICATION),
	(r#"arm/MAPI32.lib"#, ARM_MAPI32),
	(r#"arm/MDMRegistration.lib"#, ARM_MDMREGISTRATION),
	(r#"arm/MF.lib"#, ARM_MF),
	(r#"arm/MFCORE.lib"#, ARM_MFCORE),
	(r#"arm/MFPlat.lib"#, ARM_MFPLAT),
	(r#"arm/MFPlay.lib"#, ARM_MFPLAY),
	(r#"arm/MFReadWrite.lib"#, ARM_MFREADWRITE),
	(r#"arm/MFSENSORGROUP.lib"#, ARM_MFSENSORGROUP),
	(r#"arm/mfsrcsnk.lib"#, ARM_MFSRCSNK),
	(r#"arm/mgmtapi.lib"#, ARM_MGMTAPI),
	(r#"arm/mi.lib"#, ARM_MI),
	(r#"arm/MMDevAPI.lib"#, ARM_MMDEVAPI),
	(r#"arm/MPR.lib"#, ARM_MPR),
	(r#"arm/MPRAPI.lib"#, ARM_MPRAPI),
	(r#"arm/MrmSupport.lib"#, ARM_MRMSUPPORT),
	(r#"arm/MSACM32.lib"#, ARM_MSACM32),
	(r#"arm/MSAJApi.lib"#, ARM_MSAJAPI),
	(r#"arm/mscms.lib"#, ARM_MSCMS),
	(r#"arm/MsCtfMonitor.lib"#, ARM_MSCTFMONITOR),
	(r#"arm/msdmo.lib"#, ARM_MSDMO),
	(r#"arm/msdrm.lib"#, ARM_MSDRM),
	(r#"arm/msi.lib"#, ARM_MSI),
	(r#"arm/MSIMG32.lib"#, ARM_MSIMG32),
	(r#"arm/MSPORTS.lib"#, ARM_MSPORTS),
	(r#"arm/mstask.lib"#, ARM_MSTASK),
	(r#"arm/MSVFW32.lib"#, ARM_MSVFW32),
	(r#"arm/MSWSOCK.lib"#, ARM_MSWSOCK),
	(r#"arm/MTxDM.lib"#, ARM_MTXDM),
	(r#"arm/ncrypt.lib"#, ARM_NCRYPT),
	(r#"arm/NDFAPI.lib"#, ARM_NDFAPI),
	(r#"arm/NETAPI32.lib"#, ARM_NETAPI32),
	(r#"arm/NETSH.lib"#, ARM_NETSH),
	(r#"arm/newdev.lib"#, ARM_NEWDEV),
	(r#"arm/NInput.lib"#, ARM_NINPUT),
	(r#"arm/NORMALIZ.lib"#, ARM_NORMALIZ),
	(r#"arm/ntdll.lib"#, ARM_NTDLL),
	(r#"arm/NTDSAPI.lib"#, ARM_NTDSAPI),
	(r#"arm/NTLANMAN.lib"#, ARM_NTLANMAN),
	(r#"arm/OLE32.lib"#, ARM_OLE32),
	(r#"arm/OLEACC.lib"#, ARM_OLEACC),
	(r#"arm/OLEAUT32.lib"#, ARM_OLEAUT32),
	(r#"arm/oledlg.lib"#, ARM_OLEDLG),
	(r#"arm/OnDemandConnRouteHelper.lib"#, ARM_ONDEMANDCONNROUTEHELPER),
	(r#"arm/OPENGL32.lib"#, ARM_OPENGL32),
	(r#"arm/P2P.lib"#, ARM_P2P),
	(r#"arm/P2PGRAPH.lib"#, ARM_P2PGRAPH),
	(r#"arm/pdh.lib"#, ARM_PDH),
	(r#"arm/PeerDist.lib"#, ARM_PEERDIST),
	(r#"arm/POWRPROF.lib"#, ARM_POWRPROF),
	(r#"arm/prntvpt.lib"#, ARM_PRNTVPT),
	(r#"arm/PROJECTEDFSLIB.lib"#, ARM_PROJECTEDFSLIB),
	(r#"arm/PROPSYS.lib"#, ARM_PROPSYS),
	(r#"arm/QUARTZ.lib"#, ARM_QUARTZ),
	(r#"arm/query.lib"#, ARM_QUERY),
	(r#"arm/qwave.lib"#, ARM_QWAVE),
	(r#"arm/RASAPI32.lib"#, ARM_RASAPI32),
	(r#"arm/RASDLG.lib"#, ARM_RASDLG),
	(r#"arm/RESUTILS.lib"#, ARM_RESUTILS),
	(r#"arm/RoMetadata.lib"#, ARM_ROMETADATA),
	(r#"arm/RPCNS4.lib"#, ARM_RPCNS4),
	(r#"arm/RPCRT4.lib"#, ARM_RPCRT4),
	(r#"arm/RstrtMgr.lib"#, ARM_RSTRTMGR),
	(r#"arm/rtm.lib"#, ARM_RTM),
	(r#"arm/SAS.lib"#, ARM_SAS),
	(r#"arm/SCARDDLG.lib"#, ARM_SCARDDLG),
	(r#"arm/SCHANNEL.lib"#, ARM_SCHANNEL),
	(r#"arm/SECUR32.lib"#, ARM_SECUR32),
	(r#"arm/SensApi.lib"#, ARM_SENSAPI),
	(r#"arm/SETUPAPI.lib"#, ARM_SETUPAPI),
	(r#"arm/sfc.lib"#, ARM_SFC),
	(r#"arm/SHDOCVW.lib"#, ARM_SHDOCVW),
	(r#"arm/SHELL32.lib"#, ARM_SHELL32),
	(r#"arm/SHLWAPI.lib"#, ARM_SHLWAPI),
	(r#"arm/SLC.lib"#, ARM_SLC),
	(r#"arm/slcext.lib"#, ARM_SLCEXT),
	(r#"arm/SLWGA.lib"#, ARM_SLWGA),
	(r#"arm/snmpapi.lib"#, ARM_SNMPAPI),
	(r#"arm/srpapi.lib"#, ARM_SRPAPI),
	(r#"arm/SspiCli.lib"#, ARM_SSPICLI),
	(r#"arm/t2embed.lib"#, ARM_T2EMBED),
	(r#"arm/TAPI32.lib"#, ARM_TAPI32),
	(r#"arm/tbs.lib"#, ARM_TBS),
	(r#"arm/TDH.lib"#, ARM_TDH),
	(r#"arm/TOKENBINDING.lib"#, ARM_TOKENBINDING),
	(r#"arm/TRAFFIC.lib"#, ARM_TRAFFIC),
	(r#"arm/txfw32.lib"#, ARM_TXFW32),
	(r#"arm/ualapi.lib"#, ARM_UALAPI),
	(r#"arm/UIAutomationCore.lib"#, ARM_UIAUTOMATIONCORE),
	(r#"arm/URLMON.lib"#, ARM_URLMON),
	(r#"arm/USER32.lib"#, ARM_USER32),
	(r#"arm/USERENV.lib"#, ARM_USERENV),
	(r#"arm/USP10.lib"#, ARM_USP10),
	(r#"arm/UXTHEME.lib"#, ARM_UXTHEME),
	(r#"arm/verifier.lib"#, ARM_VERIFIER),
	(r#"arm/VERSION.lib"#, ARM_VERSION),
	(r#"arm/vertdll.lib"#, ARM_VERTDLL),
	(r#"arm/VirtDisk.lib"#, ARM_VIRTDISK),
	(r#"arm/VSSAPI.lib"#, ARM_VSSAPI),
	(r#"arm/wcmapi.lib"#, ARM_WCMAPI),
	(r#"arm/WDSBP.lib"#, ARM_WDSBP),
	(r#"arm/WDSCLIENTAPI.lib"#, ARM_WDSCLIENTAPI),
	(r#"arm/WDSMC.lib"#, ARM_WDSMC),
	(r#"arm/WDSPXE.lib"#, ARM_WDSPXE),
	(r#"arm/WDSTPTC.lib"#, ARM_WDSTPTC),
	(r#"arm/webauthn.lib"#, ARM_WEBAUTHN),
	(r#"arm/webservices.lib"#, ARM_WEBSERVICES),
	(r#"arm/websocket.lib"#, ARM_WEBSOCKET),
	(r#"arm/WecApi.lib"#, ARM_WECAPI),
	(r#"arm/wer.lib"#, ARM_WER),
	(r#"arm/wevtapi.lib"#, ARM_WEVTAPI),
	(r#"arm/winbio.lib"#, ARM_WINBIO),
	(r#"arm/windows.ai.machinelearning.lib"#, ARM_WINDOWS_AI_MACHINELEARNING),
	(r#"arm/Windows.Data.Pdf.lib"#, ARM_WINDOWS_DATA_PDF),
	(r#"arm/Windows.Networking.lib"#, ARM_WINDOWS_NETWORKING),
	(r#"arm/Windows.UI.Xaml.lib"#, ARM_WINDOWS_UI_XAML),
	(r#"arm/WindowsCodecs.lib"#, ARM_WINDOWSCODECS),
	(r#"arm/WINFAX.lib"#, ARM_WINFAX),
	(r#"arm/WINHTTP.lib"#, ARM_WINHTTP),
	(r#"arm/WinHvEmulation.lib"#, ARM_WINHVEMULATION),
	(r#"arm/WinHvPlatform.lib"#, ARM_WINHVPLATFORM),
	(r#"arm/WININET.lib"#, ARM_WININET),
	(r#"arm/winml.lib"#, ARM_WINML),
	(r#"arm/WINMM.lib"#, ARM_WINMM),
	(r#"arm/WinSCard.lib"#, ARM_WINSCARD),
	(r#"arm/WINSPOOL.lib"#, ARM_WINSPOOL),
	(r#"arm/WINTRUST.lib"#, ARM_WINTRUST),
	(r#"arm/WINUSB.lib"#, ARM_WINUSB),
	(r#"arm/wlanapi.lib"#, ARM_WLANAPI),
	(r#"arm/wlanui.lib"#, ARM_WLANUI),
	(r#"arm/WLDAP32.lib"#, ARM_WLDAP32),
	(r#"arm/Wldp.lib"#, ARM_WLDP),
	(r#"arm/WMVCore.lib"#, ARM_WMVCORE),
	(r#"arm/wnvapi.lib"#, ARM_WNVAPI),
	(r#"arm/WOFUTIL.lib"#, ARM_WOFUTIL),
	(r#"arm/WS2_32.lib"#, ARM_WS2_32),
	(r#"arm/WSCAPI.lib"#, ARM_WSCAPI),
	(r#"arm/WSClient.lib"#, ARM_WSCLIENT),
	(r#"arm/wsdapi.lib"#, ARM_WSDAPI),
	(r#"arm/WsmSvc.lib"#, ARM_WSMSVC),
	(r#"arm/wsnmp32.lib"#, ARM_WSNMP32),
	(r#"arm/WTSAPI32.lib"#, ARM_WTSAPI32),
	(r#"arm/XAudio2_8.lib"#, ARM_XAUDIO2_8),
	(r#"arm/XINPUTUAP.lib"#, ARM_XINPUTUAP),
	(r#"arm/XmlLite.lib"#, ARM_XMLLITE),
	(r#"arm64/ACLUI.lib"#, ARM64_ACLUI),
	(r#"arm64/ACTIVEDS.lib"#, ARM64_ACTIVEDS),
	(r#"arm64/ADVAPI32.lib"#, ARM64_ADVAPI32),
	(r#"arm64/ADVPACK.lib"#, ARM64_ADVPACK),
	(r#"arm64/Amsi.lib"#, ARM64_AMSI),
	(r#"arm64/api-ms-win-appmodel-runtime-l1-1-1.lib"#, ARM64_API_MS_WIN_APPMODEL_RUNTIME_L1_1_1),
	(r#"arm64/api-ms-win-appmodel-runtime-l1-1-3.lib"#, ARM64_API_MS_WIN_APPMODEL_RUNTIME_L1_1_3),
	(r#"arm64/api-ms-win-core-apiquery-l2-1-0.lib"#, ARM64_API_MS_WIN_CORE_APIQUERY_L2_1_0),
	(r#"arm64/api-ms-win-core-backgroundtask-l1-1-0.lib"#, ARM64_API_MS_WIN_CORE_BACKGROUNDTASK_L1_1_0),
	(r#"arm64/api-ms-win-core-comm-l1-1-1.lib"#, ARM64_API_MS_WIN_CORE_COMM_L1_1_1),
	(r#"arm64/api-ms-win-core-comm-l1-1-2.lib"#, ARM64_API_MS_WIN_CORE_COMM_L1_1_2),
	(r#"arm64/api-ms-win-core-errorhandling-l1-1-3.lib"#, ARM64_API_MS_WIN_CORE_ERRORHANDLING_L1_1_3),
	(r#"arm64/api-ms-win-core-featurestaging-l1-1-0.lib"#, ARM64_API_MS_WIN_CORE_FEATURESTAGING_L1_1_0),
	(r#"arm64/api-ms-win-core-featurestaging-l1-1-1.lib"#, ARM64_API_MS_WIN_CORE_FEATURESTAGING_L1_1_1),
	(r#"arm64/api-ms-win-core-file-fromapp-l1-1-0.lib"#, ARM64_API_MS_WIN_CORE_FILE_FROMAPP_L1_1_0),
	(r#"arm64/api-ms-win-core-handle-l1-1-0.lib"#, ARM64_API_MS_WIN_CORE_HANDLE_L1_1_0),
	(r#"arm64/api-ms-win-core-libraryloader-l2-1-0.lib"#, ARM64_API_MS_WIN_CORE_LIBRARYLOADER_L2_1_0),
	(r#"arm64/api-ms-win-core-memory-l1-1-3.lib"#, ARM64_API_MS_WIN_CORE_MEMORY_L1_1_3),
	(r#"arm64/api-ms-win-core-memory-l1-1-4.lib"#, ARM64_API_MS_WIN_CORE_MEMORY_L1_1_4),
	(r#"arm64/api-ms-win-core-memory-l1-1-5.lib"#, ARM64_API_MS_WIN_CORE_MEMORY_L1_1_5),
	(r#"arm64/api-ms-win-core-memory-l1-1-6.lib"#, ARM64_API_MS_WIN_CORE_MEMORY_L1_1_6),
	(r#"arm64/api-ms-win-core-memory-l1-1-7.lib"#, ARM64_API_MS_WIN_CORE_MEMORY_L1_1_7),
	(r#"arm64/api-ms-win-core-path-l1-1-0.lib"#, ARM64_API_MS_WIN_CORE_PATH_L1_1_0),
	(r#"arm64/api-ms-win-core-psm-appnotify-l1-1-0.lib"#, ARM64_API_MS_WIN_CORE_PSM_APPNOTIFY_L1_1_0),
	(r#"arm64/api-ms-win-core-psm-appnotify-l1-1-1.lib"#, ARM64_API_MS_WIN_CORE_PSM_APPNOTIFY_L1_1_1),
	(r#"arm64/api-ms-win-core-realtime-l1-1-1.lib"#, ARM64_API_MS_WIN_CORE_REALTIME_L1_1_1),
	(r#"arm64/api-ms-win-core-realtime-l1-1-2.lib"#, ARM64_API_MS_WIN_CORE_REALTIME_L1_1_2),
	(r#"arm64/api-ms-win-core-slapi-l1-1-0.lib"#, ARM64_API_MS_WIN_CORE_SLAPI_L1_1_0),
	(r#"arm64/api-ms-win-core-state-helpers-l1-1-0.lib"#, ARM64_API_MS_WIN_CORE_STATE_HELPERS_L1_1_0),
	(r#"arm64/api-ms-win-core-sysinfo-l1-2-0.lib"#, ARM64_API_MS_WIN_CORE_SYSINFO_L1_2_0),
	(r#"arm64/api-ms-win-core-sysinfo-l1-2-3.lib"#, ARM64_API_MS_WIN_CORE_SYSINFO_L1_2_3),
	(r#"arm64/api-ms-win-core-sysinfo-l1-2-4.lib"#, ARM64_API_MS_WIN_CORE_SYSINFO_L1_2_4),
	(r#"arm64/api-ms-win-core-util-l1-1-1.lib"#, ARM64_API_MS_WIN_CORE_UTIL_L1_1_1),
	(r#"arm64/api-ms-win-core-winrt-error-l1-1-0.lib"#, ARM64_API_MS_WIN_CORE_WINRT_ERROR_L1_1_0),
	(r#"arm64/api-ms-win-core-winrt-error-l1-1-1.lib"#, ARM64_API_MS_WIN_CORE_WINRT_ERROR_L1_1_1),
	(r#"arm64/api-ms-win-core-winrt-l1-1-0.lib"#, ARM64_API_MS_WIN_CORE_WINRT_L1_1_0),
	(r#"arm64/api-ms-win-core-winrt-registration-l1-1-0.lib"#, ARM64_API_MS_WIN_CORE_WINRT_REGISTRATION_L1_1_0),
	(r#"arm64/api-ms-win-core-winrt-robuffer-l1-1-0.lib"#, ARM64_API_MS_WIN_CORE_WINRT_ROBUFFER_L1_1_0),
	(r#"arm64/api-ms-win-core-winrt-roparameterizediid-l1-1-0.lib"#, ARM64_API_MS_WIN_CORE_WINRT_ROPARAMETERIZEDIID_L1_1_0),
	(r#"arm64/api-ms-win-core-winrt-string-l1-1-0.lib"#, ARM64_API_MS_WIN_CORE_WINRT_STRING_L1_1_0),
	(r#"arm64/api-ms-win-core-winrt-string-l1-1-1.lib"#, ARM64_API_MS_WIN_CORE_WINRT_STRING_L1_1_1),
	(r#"arm64/api-ms-win-core-wow64-l1-1-1.lib"#, ARM64_API_MS_WIN_CORE_WOW64_L1_1_1),
	(r#"arm64/api-ms-win-devices-query-l1-1-0.lib"#, ARM64_API_MS_WIN_DEVICES_QUERY_L1_1_0),
	(r#"arm64/api-ms-win-devices-query-l1-1-1.lib"#, ARM64_API_MS_WIN_DEVICES_QUERY_L1_1_1),
	(r#"arm64/api-ms-win-dx-d3dkmt-l1-1-0.lib"#, ARM64_API_MS_WIN_DX_D3DKMT_L1_1_0),
	(r#"arm64/api-ms-win-gaming-deviceinformation-l1-1-0.lib"#, ARM64_API_MS_WIN_GAMING_DEVICEINFORMATION_L1_1_0),
	(r#"arm64/api-ms-win-gaming-expandedresources-l1-1-0.lib"#, ARM64_API_MS_WIN_GAMING_EXPANDEDRESOURCES_L1_1_0),
	(r#"arm64/api-ms-win-gaming-tcui-l1-1-0.lib"#, ARM64_API_MS_WIN_GAMING_TCUI_L1_1_0),
	(r#"arm64/api-ms-win-gaming-tcui-l1-1-1.lib"#, ARM64_API_MS_WIN_GAMING_TCUI_L1_1_1),
	(r#"arm64/api-ms-win-gaming-tcui-l1-1-2.lib"#, ARM64_API_MS_WIN_GAMING_TCUI_L1_1_2),
	(r#"arm64/api-ms-win-gaming-tcui-l1-1-3.lib"#, ARM64_API_MS_WIN_GAMING_TCUI_L1_1_3),
	(r#"arm64/api-ms-win-gaming-tcui-l1-1-4.lib"#, ARM64_API_MS_WIN_GAMING_TCUI_L1_1_4),
	(r#"arm64/api-ms-win-mm-misc-l1-1-1.lib"#, ARM64_API_MS_WIN_MM_MISC_L1_1_1),
	(r#"arm64/api-ms-win-net-isolation-l1-1-0.lib"#, ARM64_API_MS_WIN_NET_ISOLATION_L1_1_0),
	(r#"arm64/api-ms-win-security-base-l1-2-2.lib"#, ARM64_API_MS_WIN_SECURITY_BASE_L1_2_2),
	(r#"arm64/api-ms-win-security-isolatedcontainer-l1-1-0.lib"#, ARM64_API_MS_WIN_SECURITY_ISOLATEDCONTAINER_L1_1_0),
	(r#"arm64/api-ms-win-security-isolatedcontainer-l1-1-1.lib"#, ARM64_API_MS_WIN_SECURITY_ISOLATEDCONTAINER_L1_1_1),
	(r#"arm64/api-ms-win-service-core-l1-1-3.lib"#, ARM64_API_MS_WIN_SERVICE_CORE_L1_1_3),
	(r#"arm64/api-ms-win-service-core-l1-1-4.lib"#, ARM64_API_MS_WIN_SERVICE_CORE_L1_1_4),
	(r#"arm64/api-ms-win-shcore-scaling-l1-1-0.lib"#, ARM64_API_MS_WIN_SHCORE_SCALING_L1_1_0),
	(r#"arm64/api-ms-win-shcore-scaling-l1-1-1.lib"#, ARM64_API_MS_WIN_SHCORE_SCALING_L1_1_1),
	(r#"arm64/api-ms-win-shcore-scaling-l1-1-2.lib"#, ARM64_API_MS_WIN_SHCORE_SCALING_L1_1_2),
	(r#"arm64/api-ms-win-shcore-stream-winrt-l1-1-0.lib"#, ARM64_API_MS_WIN_SHCORE_STREAM_WINRT_L1_1_0),
	(r#"arm64/APPHELP.lib"#, ARM64_APPHELP),
	(r#"arm64/AUTHZ.lib"#, ARM64_AUTHZ),
	(r#"arm64/AVICAP32.lib"#, ARM64_AVICAP32),
	(r#"arm64/AVIFIL32.lib"#, ARM64_AVIFIL32),
	(r#"arm64/bcrypt.lib"#, ARM64_BCRYPT),
	(r#"arm64/BluetoothApis.lib"#, ARM64_BLUETOOTHAPIS),
	(r#"arm64/bthprops.lib"#, ARM64_BTHPROPS),
	(r#"arm64/Cabinet.lib"#, ARM64_CABINET),
	(r#"arm64/certadm.lib"#, ARM64_CERTADM),
	(r#"arm64/certpoleng.lib"#, ARM64_CERTPOLENG),
	(r#"arm64/CFGMGR32.lib"#, ARM64_CFGMGR32),
	(r#"arm64/chakra.lib"#, ARM64_CHAKRA),
	(r#"arm64/cldapi.lib"#, ARM64_CLDAPI),
	(r#"arm64/clfsw32.lib"#, ARM64_CLFSW32),
	(r#"arm64/CLUSAPI.lib"#, ARM64_CLUSAPI),
	(r#"arm64/COMCTL32.lib"#, ARM64_COMCTL32),
	(r#"arm64/COMDLG32.lib"#, ARM64_COMDLG32),
	(r#"arm64/comsvcs.lib"#, ARM64_COMSVCS),
	(r#"arm64/CoreMessaging.lib"#, ARM64_COREMESSAGING),
	(r#"arm64/credui.lib"#, ARM64_CREDUI),
	(r#"arm64/CRYPT32.lib"#, ARM64_CRYPT32),
	(r#"arm64/CRYPTNET.lib"#, ARM64_CRYPTNET),
	(r#"arm64/CRYPTUI.lib"#, ARM64_CRYPTUI),
	(r#"arm64/CRYPTXML.lib"#, ARM64_CRYPTXML),
	(r#"arm64/CSCAPI.lib"#, ARM64_CSCAPI),
	(r#"arm64/d2d1.lib"#, ARM64_D2D1),
	(r#"arm64/d3d10.lib"#, ARM64_D3D10),
	(r#"arm64/d3d10_1.lib"#, ARM64_D3D10_1),
	(r#"arm64/d3d11.lib"#, ARM64_D3D11),
	(r#"arm64/d3d12.lib"#, ARM64_D3D12),
	(r#"arm64/d3d9.lib"#, ARM64_D3D9),
	(r#"arm64/D3DCOMPILER_47.lib"#, ARM64_D3DCOMPILER_47),
	(r#"arm64/d3dcsx.lib"#, ARM64_D3DCSX),
	(r#"arm64/davclnt.lib"#, ARM64_DAVCLNT),
	(r#"arm64/dbgeng.lib"#, ARM64_DBGENG),
	(r#"arm64/dbghelp.lib"#, ARM64_DBGHELP),
	(r#"arm64/dbgmodel.lib"#, ARM64_DBGMODEL),
	(r#"arm64/DCIMAN32.lib"#, ARM64_DCIMAN32),
	(r#"arm64/dcomp.lib"#, ARM64_DCOMP),
	(r#"arm64/DDRAW.lib"#, ARM64_DDRAW),
	(r#"arm64/deviceaccess.lib"#, ARM64_DEVICEACCESS),
	(r#"arm64/dflayout.lib"#, ARM64_DFLAYOUT),
	(r#"arm64/dhcpcsvc.lib"#, ARM64_DHCPCSVC),
	(r#"arm64/dhcpcsvc6.lib"#, ARM64_DHCPCSVC6),
	(r#"arm64/DHCPSAPI.lib"#, ARM64_DHCPSAPI),
	(r#"arm64/DiagnosticDataQuery.lib"#, ARM64_DIAGNOSTICDATAQUERY),
	(r#"arm64/DINPUT8.lib"#, ARM64_DINPUT8),
	(r#"arm64/DirectML.lib"#, ARM64_DIRECTML),
	(r#"arm64/DNSAPI.lib"#, ARM64_DNSAPI),
	(r#"arm64/drt.lib"#, ARM64_DRT),
	(r#"arm64/drtprov.lib"#, ARM64_DRTPROV),
	(r#"arm64/drttransport.lib"#, ARM64_DRTTRANSPORT),
	(r#"arm64/DSOUND.lib"#, ARM64_DSOUND),
	(r#"arm64/DSPARSE.lib"#, ARM64_DSPARSE),
	(r#"arm64/dsprop.lib"#, ARM64_DSPROP),
	(r#"arm64/DSSEC.lib"#, ARM64_DSSEC),
	(r#"arm64/dsuiext.lib"#, ARM64_DSUIEXT),
	(r#"arm64/dwmapi.lib"#, ARM64_DWMAPI),
	(r#"arm64/DWrite.lib"#, ARM64_DWRITE),
	(r#"arm64/dxgi.lib"#, ARM64_DXGI),
	(r#"arm64/dxva2.lib"#, ARM64_DXVA2),
	(r#"arm64/eappcfg.lib"#, ARM64_EAPPCFG),
	(r#"arm64/eappprxy.lib"#, ARM64_EAPPPRXY),
	(r#"arm64/efswrt.lib"#, ARM64_EFSWRT),
	(r#"arm64/elscore.lib"#, ARM64_ELSCORE),
	(r#"arm64/ESENT.lib"#, ARM64_ESENT),
	(r#"arm64/EVR.lib"#, ARM64_EVR),
	(r#"arm64/faultrep.lib"#, ARM64_FAULTREP),
	(r#"arm64/fhsvcctl.lib"#, ARM64_FHSVCCTL),
	(r#"arm64/FLTLIB.lib"#, ARM64_FLTLIB),
	(r#"arm64/FONTSUB.lib"#, ARM64_FONTSUB),
	(r#"arm64/fwpuclnt.lib"#, ARM64_FWPUCLNT),
	(r#"arm64/fxsutility.lib"#, ARM64_FXSUTILITY),
	(r#"arm64/GDI32.lib"#, ARM64_GDI32),
	(r#"arm64/GPEDIT.lib"#, ARM64_GPEDIT),
	(r#"arm64/HID.lib"#, ARM64_HID),
	(r#"arm64/hlink.lib"#, ARM64_HLINK),
	(r#"arm64/HrtfApo.lib"#, ARM64_HRTFAPO),
	(r#"arm64/HTTPAPI.lib"#, ARM64_HTTPAPI),
	(r#"arm64/ICM32.lib"#, ARM64_ICM32),
	(r#"arm64/ICMUI.lib"#, ARM64_ICMUI),
	(r#"arm64/icu.lib"#, ARM64_ICU),
	(r#"arm64/IMM32.lib"#, ARM64_IMM32),
	(r#"arm64/inkobjcore.lib"#, ARM64_INKOBJCORE),
	(r#"arm64/IPHLPAPI.lib"#, ARM64_IPHLPAPI),
	(r#"arm64/ISCSIDSC.lib"#, ARM64_ISCSIDSC),
	(r#"arm64/KERNEL32.lib"#, ARM64_KERNEL32),
	(r#"arm64/KeyCredMgr.lib"#, ARM64_KEYCREDMGR),
	(r#"arm64/ksuser.lib"#, ARM64_KSUSER),
	(r#"arm64/ktmw32.lib"#, ARM64_KTMW32),
	(r#"arm64/loadperf.lib"#, ARM64_LOADPERF),
	(r#"arm64/MAGNIFICATION.lib"#, ARM64_MAGNIFICATION),
	(r#"arm64/MAPI32.lib"#, ARM64_MAPI32),
	(r#"arm64/MDMRegistration.lib"#, ARM64_MDMREGISTRATION),
	(r#"arm64/MF.lib"#, ARM64_MF),
	(r#"arm64/MFCORE.lib"#, ARM64_MFCORE),
	(r#"arm64/MFPlat.lib"#, ARM64_MFPLAT),
	(r#"arm64/MFPlay.lib"#, ARM64_MFPLAY),
	(r#"arm64/MFReadWrite.lib"#, ARM64_MFREADWRITE),
	(r#"arm64/MFSENSORGROUP.lib"#, ARM64_MFSENSORGROUP),
	(r#"arm64/mfsrcsnk.lib"#, ARM64_MFSRCSNK),
	(r#"arm64/mgmtapi.lib"#, ARM64_MGMTAPI),
	(r#"arm64/mi.lib"#, ARM64_MI),
	(r#"arm64/MMDevAPI.lib"#, ARM64_MMDEVAPI),
	(r#"arm64/MPR.lib"#, ARM64_MPR),
	(r#"arm64/MPRAPI.lib"#, ARM64_MPRAPI),
	(r#"arm64/MrmSupport.lib"#, ARM64_MRMSUPPORT),
	(r#"arm64/MSACM32.lib"#, ARM64_MSACM32),
	(r#"arm64/MSAJApi.lib"#, ARM64_MSAJAPI),
	(r#"arm64/mscms.lib"#, ARM64_MSCMS),
	(r#"arm64/MsCtfMonitor.lib"#, ARM64_MSCTFMONITOR),
	(r#"arm64/msdmo.lib"#, ARM64_MSDMO),
	(r#"arm64/msdrm.lib"#, ARM64_MSDRM),
	(r#"arm64/msi.lib"#, ARM64_MSI),
	(r#"arm64/MSIMG32.lib"#, ARM64_MSIMG32),
	(r#"arm64/MSPORTS.lib"#, ARM64_MSPORTS),
	(r#"arm64/mstask.lib"#, ARM64_MSTASK),
	(r#"arm64/MSVFW32.lib"#, ARM64_MSVFW32),
	(r#"arm64/MSWSOCK.lib"#, ARM64_MSWSOCK),
	(r#"arm64/MTxDM.lib"#, ARM64_MTXDM),
	(r#"arm64/ncrypt.lib"#, ARM64_NCRYPT),
	(r#"arm64/NDFAPI.lib"#, ARM64_NDFAPI),
	(r#"arm64/NETAPI32.lib"#, ARM64_NETAPI32),
	(r#"arm64/NETSH.lib"#, ARM64_NETSH),
	(r#"arm64/newdev.lib"#, ARM64_NEWDEV),
	(r#"arm64/NInput.lib"#, ARM64_NINPUT),
	(r#"arm64/NORMALIZ.lib"#, ARM64_NORMALIZ),
	(r#"arm64/ntdll.lib"#, ARM64_NTDLL),
	(r#"arm64/NTDSAPI.lib"#, ARM64_NTDSAPI),
	(r#"arm64/NTLANMAN.lib"#, ARM64_NTLANMAN),
	(r#"arm64/OLE32.lib"#, ARM64_OLE32),
	(r#"arm64/OLEACC.lib"#, ARM64_OLEACC),
	(r#"arm64/OLEAUT32.lib"#, ARM64_OLEAUT32),
	(r#"arm64/oledlg.lib"#, ARM64_OLEDLG),
	(r#"arm64/OnDemandConnRouteHelper.lib"#, ARM64_ONDEMANDCONNROUTEHELPER),
	(r#"arm64/OPENGL32.lib"#, ARM64_OPENGL32),
	(r#"arm64/P2P.lib"#, ARM64_P2P),
	(r#"arm64/P2PGRAPH.lib"#, ARM64_P2PGRAPH),
	(r#"arm64/pdh.lib"#, ARM64_PDH),
	(r#"arm64/PeerDist.lib"#, ARM64_PEERDIST),
	(r#"arm64/POWRPROF.lib"#, ARM64_POWRPROF),
	(r#"arm64/prntvpt.lib"#, ARM64_PRNTVPT),
	(r#"arm64/PROJECTEDFSLIB.lib"#, ARM64_PROJECTEDFSLIB),
	(r#"arm64/PROPSYS.lib"#, ARM64_PROPSYS),
	(r#"arm64/QUARTZ.lib"#, ARM64_QUARTZ),
	(r#"arm64/query.lib"#, ARM64_QUERY),
	(r#"arm64/qwave.lib"#, ARM64_QWAVE),
	(r#"arm64/RASAPI32.lib"#, ARM64_RASAPI32),
	(r#"arm64/RASDLG.lib"#, ARM64_RASDLG),
	(r#"arm64/RESUTILS.lib"#, ARM64_RESUTILS),
	(r#"arm64/RoMetadata.lib"#, ARM64_ROMETADATA),
	(r#"arm64/RPCNS4.lib"#, ARM64_RPCNS4),
	(r#"arm64/RPCRT4.lib"#, ARM64_RPCRT4),
	(r#"arm64/RstrtMgr.lib"#, ARM64_RSTRTMGR),
	(r#"arm64/rtm.lib"#, ARM64_RTM),
	(r#"arm64/SAS.lib"#, ARM64_SAS),
	(r#"arm64/SCARDDLG.lib"#, ARM64_SCARDDLG),
	(r#"arm64/SCHANNEL.lib"#, ARM64_SCHANNEL),
	(r#"arm64/SECUR32.lib"#, ARM64_SECUR32),
	(r#"arm64/SensApi.lib"#, ARM64_SENSAPI),
	(r#"arm64/SETUPAPI.lib"#, ARM64_SETUPAPI),
	(r#"arm64/sfc.lib"#, ARM64_SFC),
	(r#"arm64/SHDOCVW.lib"#, ARM64_SHDOCVW),
	(r#"arm64/SHELL32.lib"#, ARM64_SHELL32),
	(r#"arm64/SHLWAPI.lib"#, ARM64_SHLWAPI),
	(r#"arm64/SLC.lib"#, ARM64_SLC),
	(r#"arm64/slcext.lib"#, ARM64_SLCEXT),
	(r#"arm64/SLWGA.lib"#, ARM64_SLWGA),
	(r#"arm64/snmpapi.lib"#, ARM64_SNMPAPI),
	(r#"arm64/srpapi.lib"#, ARM64_SRPAPI),
	(r#"arm64/SspiCli.lib"#, ARM64_SSPICLI),
	(r#"arm64/t2embed.lib"#, ARM64_T2EMBED),
	(r#"arm64/TAPI32.lib"#, ARM64_TAPI32),
	(r#"arm64/tbs.lib"#, ARM64_TBS),
	(r#"arm64/TDH.lib"#, ARM64_TDH),
	(r#"arm64/TOKENBINDING.lib"#, ARM64_TOKENBINDING),
	(r#"arm64/TRAFFIC.lib"#, ARM64_TRAFFIC),
	(r#"arm64/txfw32.lib"#, ARM64_TXFW32),
	(r#"arm64/ualapi.lib"#, ARM64_UALAPI),
	(r#"arm64/UIAutomationCore.lib"#, ARM64_UIAUTOMATIONCORE),
	(r#"arm64/URLMON.lib"#, ARM64_URLMON),
	(r#"arm64/USER32.lib"#, ARM64_USER32),
	(r#"arm64/USERENV.lib"#, ARM64_USERENV),
	(r#"arm64/USP10.lib"#, ARM64_USP10),
	(r#"arm64/UXTHEME.lib"#, ARM64_UXTHEME),
	(r#"arm64/verifier.lib"#, ARM64_VERIFIER),
	(r#"arm64/VERSION.lib"#, ARM64_VERSION),
	(r#"arm64/vertdll.lib"#, ARM64_VERTDLL),
	(r#"arm64/VirtDisk.lib"#, ARM64_VIRTDISK),
	(r#"arm64/VSSAPI.lib"#, ARM64_VSSAPI),
	(r#"arm64/wcmapi.lib"#, ARM64_WCMAPI),
	(r#"arm64/WDSBP.lib"#, ARM64_WDSBP),
	(r#"arm64/WDSCLIENTAPI.lib"#, ARM64_WDSCLIENTAPI),
	(r#"arm64/WDSMC.lib"#, ARM64_WDSMC),
	(r#"arm64/WDSPXE.lib"#, ARM64_WDSPXE),
	(r#"arm64/WDSTPTC.lib"#, ARM64_WDSTPTC),
	(r#"arm64/webauthn.lib"#, ARM64_WEBAUTHN),
	(r#"arm64/webservices.lib"#, ARM64_WEBSERVICES),
	(r#"arm64/websocket.lib"#, ARM64_WEBSOCKET),
	(r#"arm64/WecApi.lib"#, ARM64_WECAPI),
	(r#"arm64/wer.lib"#, ARM64_WER),
	(r#"arm64/wevtapi.lib"#, ARM64_WEVTAPI),
	(r#"arm64/winbio.lib"#, ARM64_WINBIO),
	(r#"arm64/windows.ai.machinelearning.lib"#, ARM64_WINDOWS_AI_MACHINELEARNING),
	(r#"arm64/Windows.Data.Pdf.lib"#, ARM64_WINDOWS_DATA_PDF),
	(r#"arm64/Windows.Networking.lib"#, ARM64_WINDOWS_NETWORKING),
	(r#"arm64/Windows.UI.Xaml.lib"#, ARM64_WINDOWS_UI_XAML),
	(r#"arm64/WindowsCodecs.lib"#, ARM64_WINDOWSCODECS),
	(r#"arm64/WINFAX.lib"#, ARM64_WINFAX),
	(r#"arm64/WINHTTP.lib"#, ARM64_WINHTTP),
	(r#"arm64/WinHvEmulation.lib"#, ARM64_WINHVEMULATION),
	(r#"arm64/WinHvPlatform.lib"#, ARM64_WINHVPLATFORM),
	(r#"arm64/WININET.lib"#, ARM64_WININET),
	(r#"arm64/winml.lib"#, ARM64_WINML),
	(r#"arm64/WINMM.lib"#, ARM64_WINMM),
	(r#"arm64/WinSCard.lib"#, ARM64_WINSCARD),
	(r#"arm64/WINSPOOL.lib"#, ARM64_WINSPOOL),
	(r#"arm64/WINTRUST.lib"#, ARM64_WINTRUST),
	(r#"arm64/WINUSB.lib"#, ARM64_WINUSB),
	(r#"arm64/wlanapi.lib"#, ARM64_WLANAPI),
	(r#"arm64/wlanui.lib"#, ARM64_WLANUI),
	(r#"arm64/WLDAP32.lib"#, ARM64_WLDAP32),
	(r#"arm64/Wldp.lib"#, ARM64_WLDP),
	(r#"arm64/WMVCore.lib"#, ARM64_WMVCORE),
	(r#"arm64/wnvapi.lib"#, ARM64_WNVAPI),
	(r#"arm64/WOFUTIL.lib"#, ARM64_WOFUTIL),
	(r#"arm64/WS2_32.lib"#, ARM64_WS2_32),
	(r#"arm64/WSCAPI.lib"#, ARM64_WSCAPI),
	(r#"arm64/WSClient.lib"#, ARM64_WSCLIENT),
	(r#"arm64/wsdapi.lib"#, ARM64_WSDAPI),
	(r#"arm64/WsmSvc.lib"#, ARM64_WSMSVC),
	(r#"arm64/wsnmp32.lib"#, ARM64_WSNMP32),
	(r#"arm64/WTSAPI32.lib"#, ARM64_WTSAPI32),
	(r#"arm64/XAudio2_8.lib"#, ARM64_XAUDIO2_8),
	(r#"arm64/XINPUTUAP.lib"#, ARM64_XINPUTUAP),
	(r#"arm64/XmlLite.lib"#, ARM64_XMLLITE),
];
static X64_ACLUI: &[u8] = include_bytes!(r#"../libs\x64/ACLUI.lib"#);
static X64_ACTIVEDS: &[u8] = include_bytes!(r#"../libs\x64/ACTIVEDS.lib"#);
static X64_ADVAPI32: &[u8] = include_bytes!(r#"../libs\x64/ADVAPI32.lib"#);
static X64_ADVPACK: &[u8] = include_bytes!(r#"../libs\x64/ADVPACK.lib"#);
static X64_AMSI: &[u8] = include_bytes!(r#"../libs\x64/Amsi.lib"#);
static X64_API_MS_WIN_APPMODEL_RUNTIME_L1_1_1: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-appmodel-runtime-l1-1-1.lib"#);
static X64_API_MS_WIN_APPMODEL_RUNTIME_L1_1_3: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-appmodel-runtime-l1-1-3.lib"#);
static X64_API_MS_WIN_CORE_APIQUERY_L2_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-apiquery-l2-1-0.lib"#);
static X64_API_MS_WIN_CORE_BACKGROUNDTASK_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-backgroundtask-l1-1-0.lib"#);
static X64_API_MS_WIN_CORE_COMM_L1_1_1: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-comm-l1-1-1.lib"#);
static X64_API_MS_WIN_CORE_COMM_L1_1_2: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-comm-l1-1-2.lib"#);
static X64_API_MS_WIN_CORE_ERRORHANDLING_L1_1_3: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-errorhandling-l1-1-3.lib"#);
static X64_API_MS_WIN_CORE_FEATURESTAGING_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-featurestaging-l1-1-0.lib"#);
static X64_API_MS_WIN_CORE_FEATURESTAGING_L1_1_1: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-featurestaging-l1-1-1.lib"#);
static X64_API_MS_WIN_CORE_FILE_FROMAPP_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-file-fromapp-l1-1-0.lib"#);
static X64_API_MS_WIN_CORE_HANDLE_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-handle-l1-1-0.lib"#);
static X64_API_MS_WIN_CORE_LIBRARYLOADER_L2_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-libraryloader-l2-1-0.lib"#);
static X64_API_MS_WIN_CORE_MEMORY_L1_1_3: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-memory-l1-1-3.lib"#);
static X64_API_MS_WIN_CORE_MEMORY_L1_1_4: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-memory-l1-1-4.lib"#);
static X64_API_MS_WIN_CORE_MEMORY_L1_1_5: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-memory-l1-1-5.lib"#);
static X64_API_MS_WIN_CORE_MEMORY_L1_1_6: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-memory-l1-1-6.lib"#);
static X64_API_MS_WIN_CORE_MEMORY_L1_1_7: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-memory-l1-1-7.lib"#);
static X64_API_MS_WIN_CORE_PATH_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-path-l1-1-0.lib"#);
static X64_API_MS_WIN_CORE_PSM_APPNOTIFY_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-psm-appnotify-l1-1-0.lib"#);
static X64_API_MS_WIN_CORE_PSM_APPNOTIFY_L1_1_1: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-psm-appnotify-l1-1-1.lib"#);
static X64_API_MS_WIN_CORE_REALTIME_L1_1_1: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-realtime-l1-1-1.lib"#);
static X64_API_MS_WIN_CORE_REALTIME_L1_1_2: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-realtime-l1-1-2.lib"#);
static X64_API_MS_WIN_CORE_SLAPI_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-slapi-l1-1-0.lib"#);
static X64_API_MS_WIN_CORE_STATE_HELPERS_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-state-helpers-l1-1-0.lib"#);
static X64_API_MS_WIN_CORE_SYSINFO_L1_2_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-sysinfo-l1-2-0.lib"#);
static X64_API_MS_WIN_CORE_SYSINFO_L1_2_3: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-sysinfo-l1-2-3.lib"#);
static X64_API_MS_WIN_CORE_SYSINFO_L1_2_4: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-sysinfo-l1-2-4.lib"#);
static X64_API_MS_WIN_CORE_UTIL_L1_1_1: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-util-l1-1-1.lib"#);
static X64_API_MS_WIN_CORE_WINRT_ERROR_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-winrt-error-l1-1-0.lib"#);
static X64_API_MS_WIN_CORE_WINRT_ERROR_L1_1_1: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-winrt-error-l1-1-1.lib"#);
static X64_API_MS_WIN_CORE_WINRT_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-winrt-l1-1-0.lib"#);
static X64_API_MS_WIN_CORE_WINRT_REGISTRATION_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-winrt-registration-l1-1-0.lib"#);
static X64_API_MS_WIN_CORE_WINRT_ROBUFFER_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-winrt-robuffer-l1-1-0.lib"#);
static X64_API_MS_WIN_CORE_WINRT_ROPARAMETERIZEDIID_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-winrt-roparameterizediid-l1-1-0.lib"#);
static X64_API_MS_WIN_CORE_WINRT_STRING_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-winrt-string-l1-1-0.lib"#);
static X64_API_MS_WIN_CORE_WINRT_STRING_L1_1_1: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-winrt-string-l1-1-1.lib"#);
static X64_API_MS_WIN_CORE_WOW64_L1_1_1: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-core-wow64-l1-1-1.lib"#);
static X64_API_MS_WIN_DEVICES_QUERY_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-devices-query-l1-1-0.lib"#);
static X64_API_MS_WIN_DEVICES_QUERY_L1_1_1: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-devices-query-l1-1-1.lib"#);
static X64_API_MS_WIN_DX_D3DKMT_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-dx-d3dkmt-l1-1-0.lib"#);
static X64_API_MS_WIN_GAMING_DEVICEINFORMATION_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-gaming-deviceinformation-l1-1-0.lib"#);
static X64_API_MS_WIN_GAMING_EXPANDEDRESOURCES_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-gaming-expandedresources-l1-1-0.lib"#);
static X64_API_MS_WIN_GAMING_TCUI_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-gaming-tcui-l1-1-0.lib"#);
static X64_API_MS_WIN_GAMING_TCUI_L1_1_1: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-gaming-tcui-l1-1-1.lib"#);
static X64_API_MS_WIN_GAMING_TCUI_L1_1_2: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-gaming-tcui-l1-1-2.lib"#);
static X64_API_MS_WIN_GAMING_TCUI_L1_1_3: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-gaming-tcui-l1-1-3.lib"#);
static X64_API_MS_WIN_GAMING_TCUI_L1_1_4: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-gaming-tcui-l1-1-4.lib"#);
static X64_API_MS_WIN_MM_MISC_L1_1_1: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-mm-misc-l1-1-1.lib"#);
static X64_API_MS_WIN_NET_ISOLATION_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-net-isolation-l1-1-0.lib"#);
static X64_API_MS_WIN_SECURITY_BASE_L1_2_2: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-security-base-l1-2-2.lib"#);
static X64_API_MS_WIN_SECURITY_ISOLATEDCONTAINER_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-security-isolatedcontainer-l1-1-0.lib"#);
static X64_API_MS_WIN_SECURITY_ISOLATEDCONTAINER_L1_1_1: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-security-isolatedcontainer-l1-1-1.lib"#);
static X64_API_MS_WIN_SERVICE_CORE_L1_1_3: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-service-core-l1-1-3.lib"#);
static X64_API_MS_WIN_SERVICE_CORE_L1_1_4: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-service-core-l1-1-4.lib"#);
static X64_API_MS_WIN_SHCORE_SCALING_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-shcore-scaling-l1-1-0.lib"#);
static X64_API_MS_WIN_SHCORE_SCALING_L1_1_1: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-shcore-scaling-l1-1-1.lib"#);
static X64_API_MS_WIN_SHCORE_SCALING_L1_1_2: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-shcore-scaling-l1-1-2.lib"#);
static X64_API_MS_WIN_SHCORE_STREAM_WINRT_L1_1_0: &[u8] = include_bytes!(r#"../libs\x64/api-ms-win-shcore-stream-winrt-l1-1-0.lib"#);
static X64_APPHELP: &[u8] = include_bytes!(r#"../libs\x64/APPHELP.lib"#);
static X64_AUTHZ: &[u8] = include_bytes!(r#"../libs\x64/AUTHZ.lib"#);
static X64_AVICAP32: &[u8] = include_bytes!(r#"../libs\x64/AVICAP32.lib"#);
static X64_AVIFIL32: &[u8] = include_bytes!(r#"../libs\x64/AVIFIL32.lib"#);
static X64_BCRYPT: &[u8] = include_bytes!(r#"../libs\x64/bcrypt.lib"#);
static X64_BLUETOOTHAPIS: &[u8] = include_bytes!(r#"../libs\x64/BluetoothApis.lib"#);
static X64_BTHPROPS: &[u8] = include_bytes!(r#"../libs\x64/bthprops.lib"#);
static X64_CABINET: &[u8] = include_bytes!(r#"../libs\x64/Cabinet.lib"#);
static X64_CERTADM: &[u8] = include_bytes!(r#"../libs\x64/certadm.lib"#);
static X64_CERTPOLENG: &[u8] = include_bytes!(r#"../libs\x64/certpoleng.lib"#);
static X64_CFGMGR32: &[u8] = include_bytes!(r#"../libs\x64/CFGMGR32.lib"#);
static X64_CHAKRA: &[u8] = include_bytes!(r#"../libs\x64/chakra.lib"#);
static X64_CLDAPI: &[u8] = include_bytes!(r#"../libs\x64/cldapi.lib"#);
static X64_CLFSW32: &[u8] = include_bytes!(r#"../libs\x64/clfsw32.lib"#);
static X64_CLUSAPI: &[u8] = include_bytes!(r#"../libs\x64/CLUSAPI.lib"#);
static X64_COMCTL32: &[u8] = include_bytes!(r#"../libs\x64/COMCTL32.lib"#);
static X64_COMDLG32: &[u8] = include_bytes!(r#"../libs\x64/COMDLG32.lib"#);
static X64_COMSVCS: &[u8] = include_bytes!(r#"../libs\x64/comsvcs.lib"#);
static X64_COREMESSAGING: &[u8] = include_bytes!(r#"../libs\x64/CoreMessaging.lib"#);
static X64_CREDUI: &[u8] = include_bytes!(r#"../libs\x64/credui.lib"#);
static X64_CRYPT32: &[u8] = include_bytes!(r#"../libs\x64/CRYPT32.lib"#);
static X64_CRYPTNET: &[u8] = include_bytes!(r#"../libs\x64/CRYPTNET.lib"#);
static X64_CRYPTUI: &[u8] = include_bytes!(r#"../libs\x64/CRYPTUI.lib"#);
static X64_CRYPTXML: &[u8] = include_bytes!(r#"../libs\x64/CRYPTXML.lib"#);
static X64_CSCAPI: &[u8] = include_bytes!(r#"../libs\x64/CSCAPI.lib"#);
static X64_D2D1: &[u8] = include_bytes!(r#"../libs\x64/d2d1.lib"#);
static X64_D3D10: &[u8] = include_bytes!(r#"../libs\x64/d3d10.lib"#);
static X64_D3D10_1: &[u8] = include_bytes!(r#"../libs\x64/d3d10_1.lib"#);
static X64_D3D11: &[u8] = include_bytes!(r#"../libs\x64/d3d11.lib"#);
static X64_D3D12: &[u8] = include_bytes!(r#"../libs\x64/d3d12.lib"#);
static X64_D3D9: &[u8] = include_bytes!(r#"../libs\x64/d3d9.lib"#);
static X64_D3DCOMPILER_47: &[u8] = include_bytes!(r#"../libs\x64/D3DCOMPILER_47.lib"#);
static X64_D3DCSX: &[u8] = include_bytes!(r#"../libs\x64/d3dcsx.lib"#);
static X64_DAVCLNT: &[u8] = include_bytes!(r#"../libs\x64/davclnt.lib"#);
static X64_DBGENG: &[u8] = include_bytes!(r#"../libs\x64/dbgeng.lib"#);
static X64_DBGHELP: &[u8] = include_bytes!(r#"../libs\x64/dbghelp.lib"#);
static X64_DBGMODEL: &[u8] = include_bytes!(r#"../libs\x64/dbgmodel.lib"#);
static X64_DCIMAN32: &[u8] = include_bytes!(r#"../libs\x64/DCIMAN32.lib"#);
static X64_DCOMP: &[u8] = include_bytes!(r#"../libs\x64/dcomp.lib"#);
static X64_DDRAW: &[u8] = include_bytes!(r#"../libs\x64/DDRAW.lib"#);
static X64_DEVICEACCESS: &[u8] = include_bytes!(r#"../libs\x64/deviceaccess.lib"#);
static X64_DFLAYOUT: &[u8] = include_bytes!(r#"../libs\x64/dflayout.lib"#);
static X64_DHCPCSVC: &[u8] = include_bytes!(r#"../libs\x64/dhcpcsvc.lib"#);
static X64_DHCPCSVC6: &[u8] = include_bytes!(r#"../libs\x64/dhcpcsvc6.lib"#);
static X64_DHCPSAPI: &[u8] = include_bytes!(r#"../libs\x64/DHCPSAPI.lib"#);
static X64_DIAGNOSTICDATAQUERY: &[u8] = include_bytes!(r#"../libs\x64/DiagnosticDataQuery.lib"#);
static X64_DINPUT8: &[u8] = include_bytes!(r#"../libs\x64/DINPUT8.lib"#);
static X64_DIRECTML: &[u8] = include_bytes!(r#"../libs\x64/DirectML.lib"#);
static X64_DNSAPI: &[u8] = include_bytes!(r#"../libs\x64/DNSAPI.lib"#);
static X64_DRT: &[u8] = include_bytes!(r#"../libs\x64/drt.lib"#);
static X64_DRTPROV: &[u8] = include_bytes!(r#"../libs\x64/drtprov.lib"#);
static X64_DRTTRANSPORT: &[u8] = include_bytes!(r#"../libs\x64/drttransport.lib"#);
static X64_DSOUND: &[u8] = include_bytes!(r#"../libs\x64/DSOUND.lib"#);
static X64_DSPARSE: &[u8] = include_bytes!(r#"../libs\x64/DSPARSE.lib"#);
static X64_DSPROP: &[u8] = include_bytes!(r#"../libs\x64/dsprop.lib"#);
static X64_DSSEC: &[u8] = include_bytes!(r#"../libs\x64/DSSEC.lib"#);
static X64_DSUIEXT: &[u8] = include_bytes!(r#"../libs\x64/dsuiext.lib"#);
static X64_DWMAPI: &[u8] = include_bytes!(r#"../libs\x64/dwmapi.lib"#);
static X64_DWRITE: &[u8] = include_bytes!(r#"../libs\x64/DWrite.lib"#);
static X64_DXGI: &[u8] = include_bytes!(r#"../libs\x64/dxgi.lib"#);
static X64_DXVA2: &[u8] = include_bytes!(r#"../libs\x64/dxva2.lib"#);
static X64_EAPPCFG: &[u8] = include_bytes!(r#"../libs\x64/eappcfg.lib"#);
static X64_EAPPPRXY: &[u8] = include_bytes!(r#"../libs\x64/eappprxy.lib"#);
static X64_EFSWRT: &[u8] = include_bytes!(r#"../libs\x64/efswrt.lib"#);
static X64_ELSCORE: &[u8] = include_bytes!(r#"../libs\x64/elscore.lib"#);
static X64_ESENT: &[u8] = include_bytes!(r#"../libs\x64/ESENT.lib"#);
static X64_EVR: &[u8] = include_bytes!(r#"../libs\x64/EVR.lib"#);
static X64_FAULTREP: &[u8] = include_bytes!(r#"../libs\x64/faultrep.lib"#);
static X64_FHSVCCTL: &[u8] = include_bytes!(r#"../libs\x64/fhsvcctl.lib"#);
static X64_FLTLIB: &[u8] = include_bytes!(r#"../libs\x64/FLTLIB.lib"#);
static X64_FONTSUB: &[u8] = include_bytes!(r#"../libs\x64/FONTSUB.lib"#);
static X64_FWPUCLNT: &[u8] = include_bytes!(r#"../libs\x64/fwpuclnt.lib"#);
static X64_FXSUTILITY: &[u8] = include_bytes!(r#"../libs\x64/fxsutility.lib"#);
static X64_GDI32: &[u8] = include_bytes!(r#"../libs\x64/GDI32.lib"#);
static X64_GPEDIT: &[u8] = include_bytes!(r#"../libs\x64/GPEDIT.lib"#);
static X64_HID: &[u8] = include_bytes!(r#"../libs\x64/HID.lib"#);
static X64_HLINK: &[u8] = include_bytes!(r#"../libs\x64/hlink.lib"#);
static X64_HRTFAPO: &[u8] = include_bytes!(r#"../libs\x64/HrtfApo.lib"#);
static X64_HTTPAPI: &[u8] = include_bytes!(r#"../libs\x64/HTTPAPI.lib"#);
static X64_ICM32: &[u8] = include_bytes!(r#"../libs\x64/ICM32.lib"#);
static X64_ICMUI: &[u8] = include_bytes!(r#"../libs\x64/ICMUI.lib"#);
static X64_ICU: &[u8] = include_bytes!(r#"../libs\x64/icu.lib"#);
static X64_IMM32: &[u8] = include_bytes!(r#"../libs\x64/IMM32.lib"#);
static X64_INKOBJCORE: &[u8] = include_bytes!(r#"../libs\x64/inkobjcore.lib"#);
static X64_IPHLPAPI: &[u8] = include_bytes!(r#"../libs\x64/IPHLPAPI.lib"#);
static X64_ISCSIDSC: &[u8] = include_bytes!(r#"../libs\x64/ISCSIDSC.lib"#);
static X64_KERNEL32: &[u8] = include_bytes!(r#"../libs\x64/KERNEL32.lib"#);
static X64_KEYCREDMGR: &[u8] = include_bytes!(r#"../libs\x64/KeyCredMgr.lib"#);
static X64_KSUSER: &[u8] = include_bytes!(r#"../libs\x64/ksuser.lib"#);
static X64_KTMW32: &[u8] = include_bytes!(r#"../libs\x64/ktmw32.lib"#);
static X64_LOADPERF: &[u8] = include_bytes!(r#"../libs\x64/loadperf.lib"#);
static X64_MAGNIFICATION: &[u8] = include_bytes!(r#"../libs\x64/MAGNIFICATION.lib"#);
static X64_MAPI32: &[u8] = include_bytes!(r#"../libs\x64/MAPI32.lib"#);
static X64_MDMREGISTRATION: &[u8] = include_bytes!(r#"../libs\x64/MDMRegistration.lib"#);
static X64_MF: &[u8] = include_bytes!(r#"../libs\x64/MF.lib"#);
static X64_MFCORE: &[u8] = include_bytes!(r#"../libs\x64/MFCORE.lib"#);
static X64_MFPLAT: &[u8] = include_bytes!(r#"../libs\x64/MFPlat.lib"#);
static X64_MFPLAY: &[u8] = include_bytes!(r#"../libs\x64/MFPlay.lib"#);
static X64_MFREADWRITE: &[u8] = include_bytes!(r#"../libs\x64/MFReadWrite.lib"#);
static X64_MFSENSORGROUP: &[u8] = include_bytes!(r#"../libs\x64/MFSENSORGROUP.lib"#);
static X64_MFSRCSNK: &[u8] = include_bytes!(r#"../libs\x64/mfsrcsnk.lib"#);
static X64_MGMTAPI: &[u8] = include_bytes!(r#"../libs\x64/mgmtapi.lib"#);
static X64_MI: &[u8] = include_bytes!(r#"../libs\x64/mi.lib"#);
static X64_MMDEVAPI: &[u8] = include_bytes!(r#"../libs\x64/MMDevAPI.lib"#);
static X64_MPR: &[u8] = include_bytes!(r#"../libs\x64/MPR.lib"#);
static X64_MPRAPI: &[u8] = include_bytes!(r#"../libs\x64/MPRAPI.lib"#);
static X64_MRMSUPPORT: &[u8] = include_bytes!(r#"../libs\x64/MrmSupport.lib"#);
static X64_MSACM32: &[u8] = include_bytes!(r#"../libs\x64/MSACM32.lib"#);
static X64_MSAJAPI: &[u8] = include_bytes!(r#"../libs\x64/MSAJApi.lib"#);
static X64_MSCMS: &[u8] = include_bytes!(r#"../libs\x64/mscms.lib"#);
static X64_MSCTFMONITOR: &[u8] = include_bytes!(r#"../libs\x64/MsCtfMonitor.lib"#);
static X64_MSDMO: &[u8] = include_bytes!(r#"../libs\x64/msdmo.lib"#);
static X64_MSDRM: &[u8] = include_bytes!(r#"../libs\x64/msdrm.lib"#);
static X64_MSI: &[u8] = include_bytes!(r#"../libs\x64/msi.lib"#);
static X64_MSIMG32: &[u8] = include_bytes!(r#"../libs\x64/MSIMG32.lib"#);
static X64_MSPORTS: &[u8] = include_bytes!(r#"../libs\x64/MSPORTS.lib"#);
static X64_MSTASK: &[u8] = include_bytes!(r#"../libs\x64/mstask.lib"#);
static X64_MSVFW32: &[u8] = include_bytes!(r#"../libs\x64/MSVFW32.lib"#);
static X64_MSWSOCK: &[u8] = include_bytes!(r#"../libs\x64/MSWSOCK.lib"#);
static X64_MTXDM: &[u8] = include_bytes!(r#"../libs\x64/MTxDM.lib"#);
static X64_NCRYPT: &[u8] = include_bytes!(r#"../libs\x64/ncrypt.lib"#);
static X64_NDFAPI: &[u8] = include_bytes!(r#"../libs\x64/NDFAPI.lib"#);
static X64_NETAPI32: &[u8] = include_bytes!(r#"../libs\x64/NETAPI32.lib"#);
static X64_NETSH: &[u8] = include_bytes!(r#"../libs\x64/NETSH.lib"#);
static X64_NEWDEV: &[u8] = include_bytes!(r#"../libs\x64/newdev.lib"#);
static X64_NINPUT: &[u8] = include_bytes!(r#"../libs\x64/NInput.lib"#);
static X64_NORMALIZ: &[u8] = include_bytes!(r#"../libs\x64/NORMALIZ.lib"#);
static X64_NTDLL: &[u8] = include_bytes!(r#"../libs\x64/ntdll.lib"#);
static X64_NTDSAPI: &[u8] = include_bytes!(r#"../libs\x64/NTDSAPI.lib"#);
static X64_NTLANMAN: &[u8] = include_bytes!(r#"../libs\x64/NTLANMAN.lib"#);
static X64_OLE32: &[u8] = include_bytes!(r#"../libs\x64/OLE32.lib"#);
static X64_OLEACC: &[u8] = include_bytes!(r#"../libs\x64/OLEACC.lib"#);
static X64_OLEAUT32: &[u8] = include_bytes!(r#"../libs\x64/OLEAUT32.lib"#);
static X64_OLEDLG: &[u8] = include_bytes!(r#"../libs\x64/oledlg.lib"#);
static X64_ONDEMANDCONNROUTEHELPER: &[u8] = include_bytes!(r#"../libs\x64/OnDemandConnRouteHelper.lib"#);
static X64_OPENGL32: &[u8] = include_bytes!(r#"../libs\x64/OPENGL32.lib"#);
static X64_P2P: &[u8] = include_bytes!(r#"../libs\x64/P2P.lib"#);
static X64_P2PGRAPH: &[u8] = include_bytes!(r#"../libs\x64/P2PGRAPH.lib"#);
static X64_PDH: &[u8] = include_bytes!(r#"../libs\x64/pdh.lib"#);
static X64_PEERDIST: &[u8] = include_bytes!(r#"../libs\x64/PeerDist.lib"#);
static X64_POWRPROF: &[u8] = include_bytes!(r#"../libs\x64/POWRPROF.lib"#);
static X64_PRNTVPT: &[u8] = include_bytes!(r#"../libs\x64/prntvpt.lib"#);
static X64_PROJECTEDFSLIB: &[u8] = include_bytes!(r#"../libs\x64/PROJECTEDFSLIB.lib"#);
static X64_PROPSYS: &[u8] = include_bytes!(r#"../libs\x64/PROPSYS.lib"#);
static X64_QUARTZ: &[u8] = include_bytes!(r#"../libs\x64/QUARTZ.lib"#);
static X64_QUERY: &[u8] = include_bytes!(r#"../libs\x64/query.lib"#);
static X64_QWAVE: &[u8] = include_bytes!(r#"../libs\x64/qwave.lib"#);
static X64_RASAPI32: &[u8] = include_bytes!(r#"../libs\x64/RASAPI32.lib"#);
static X64_RASDLG: &[u8] = include_bytes!(r#"../libs\x64/RASDLG.lib"#);
static X64_RESUTILS: &[u8] = include_bytes!(r#"../libs\x64/RESUTILS.lib"#);
static X64_ROMETADATA: &[u8] = include_bytes!(r#"../libs\x64/RoMetadata.lib"#);
static X64_RPCNS4: &[u8] = include_bytes!(r#"../libs\x64/RPCNS4.lib"#);
static X64_RPCRT4: &[u8] = include_bytes!(r#"../libs\x64/RPCRT4.lib"#);
static X64_RSTRTMGR: &[u8] = include_bytes!(r#"../libs\x64/RstrtMgr.lib"#);
static X64_RTM: &[u8] = include_bytes!(r#"../libs\x64/rtm.lib"#);
static X64_SAS: &[u8] = include_bytes!(r#"../libs\x64/SAS.lib"#);
static X64_SCARDDLG: &[u8] = include_bytes!(r#"../libs\x64/SCARDDLG.lib"#);
static X64_SCHANNEL: &[u8] = include_bytes!(r#"../libs\x64/SCHANNEL.lib"#);
static X64_SECUR32: &[u8] = include_bytes!(r#"../libs\x64/SECUR32.lib"#);
static X64_SENSAPI: &[u8] = include_bytes!(r#"../libs\x64/SensApi.lib"#);
static X64_SETUPAPI: &[u8] = include_bytes!(r#"../libs\x64/SETUPAPI.lib"#);
static X64_SFC: &[u8] = include_bytes!(r#"../libs\x64/sfc.lib"#);
static X64_SHDOCVW: &[u8] = include_bytes!(r#"../libs\x64/SHDOCVW.lib"#);
static X64_SHELL32: &[u8] = include_bytes!(r#"../libs\x64/SHELL32.lib"#);
static X64_SHLWAPI: &[u8] = include_bytes!(r#"../libs\x64/SHLWAPI.lib"#);
static X64_SLC: &[u8] = include_bytes!(r#"../libs\x64/SLC.lib"#);
static X64_SLCEXT: &[u8] = include_bytes!(r#"../libs\x64/slcext.lib"#);
static X64_SLWGA: &[u8] = include_bytes!(r#"../libs\x64/SLWGA.lib"#);
static X64_SNMPAPI: &[u8] = include_bytes!(r#"../libs\x64/snmpapi.lib"#);
static X64_SRPAPI: &[u8] = include_bytes!(r#"../libs\x64/srpapi.lib"#);
static X64_SSPICLI: &[u8] = include_bytes!(r#"../libs\x64/SspiCli.lib"#);
static X64_T2EMBED: &[u8] = include_bytes!(r#"../libs\x64/t2embed.lib"#);
static X64_TAPI32: &[u8] = include_bytes!(r#"../libs\x64/TAPI32.lib"#);
static X64_TBS: &[u8] = include_bytes!(r#"../libs\x64/tbs.lib"#);
static X64_TDH: &[u8] = include_bytes!(r#"../libs\x64/TDH.lib"#);
static X64_TOKENBINDING: &[u8] = include_bytes!(r#"../libs\x64/TOKENBINDING.lib"#);
static X64_TRAFFIC: &[u8] = include_bytes!(r#"../libs\x64/TRAFFIC.lib"#);
static X64_TXFW32: &[u8] = include_bytes!(r#"../libs\x64/txfw32.lib"#);
static X64_UALAPI: &[u8] = include_bytes!(r#"../libs\x64/ualapi.lib"#);
static X64_UIAUTOMATIONCORE: &[u8] = include_bytes!(r#"../libs\x64/UIAutomationCore.lib"#);
static X64_URLMON: &[u8] = include_bytes!(r#"../libs\x64/URLMON.lib"#);
static X64_USER32: &[u8] = include_bytes!(r#"../libs\x64/USER32.lib"#);
static X64_USERENV: &[u8] = include_bytes!(r#"../libs\x64/USERENV.lib"#);
static X64_USP10: &[u8] = include_bytes!(r#"../libs\x64/USP10.lib"#);
static X64_UXTHEME: &[u8] = include_bytes!(r#"../libs\x64/UXTHEME.lib"#);
static X64_VERIFIER: &[u8] = include_bytes!(r#"../libs\x64/verifier.lib"#);
static X64_VERSION: &[u8] = include_bytes!(r#"../libs\x64/VERSION.lib"#);
static X64_VERTDLL: &[u8] = include_bytes!(r#"../libs\x64/vertdll.lib"#);
static X64_VIRTDISK: &[u8] = include_bytes!(r#"../libs\x64/VirtDisk.lib"#);
static X64_VSSAPI: &[u8] = include_bytes!(r#"../libs\x64/VSSAPI.lib"#);
static X64_WCMAPI: &[u8] = include_bytes!(r#"../libs\x64/wcmapi.lib"#);
static X64_WDSBP: &[u8] = include_bytes!(r#"../libs\x64/WDSBP.lib"#);
static X64_WDSCLIENTAPI: &[u8] = include_bytes!(r#"../libs\x64/WDSCLIENTAPI.lib"#);
static X64_WDSMC: &[u8] = include_bytes!(r#"../libs\x64/WDSMC.lib"#);
static X64_WDSPXE: &[u8] = include_bytes!(r#"../libs\x64/WDSPXE.lib"#);
static X64_WDSTPTC: &[u8] = include_bytes!(r#"../libs\x64/WDSTPTC.lib"#);
static X64_WEBAUTHN: &[u8] = include_bytes!(r#"../libs\x64/webauthn.lib"#);
static X64_WEBSERVICES: &[u8] = include_bytes!(r#"../libs\x64/webservices.lib"#);
static X64_WEBSOCKET: &[u8] = include_bytes!(r#"../libs\x64/websocket.lib"#);
static X64_WECAPI: &[u8] = include_bytes!(r#"../libs\x64/WecApi.lib"#);
static X64_WER: &[u8] = include_bytes!(r#"../libs\x64/wer.lib"#);
static X64_WEVTAPI: &[u8] = include_bytes!(r#"../libs\x64/wevtapi.lib"#);
static X64_WINBIO: &[u8] = include_bytes!(r#"../libs\x64/winbio.lib"#);
static X64_WINDOWS_AI_MACHINELEARNING: &[u8] = include_bytes!(r#"../libs\x64/windows.ai.machinelearning.lib"#);
static X64_WINDOWS_DATA_PDF: &[u8] = include_bytes!(r#"../libs\x64/Windows.Data.Pdf.lib"#);
static X64_WINDOWS_NETWORKING: &[u8] = include_bytes!(r#"../libs\x64/Windows.Networking.lib"#);
static X64_WINDOWS_UI_XAML: &[u8] = include_bytes!(r#"../libs\x64/Windows.UI.Xaml.lib"#);
static X64_WINDOWSCODECS: &[u8] = include_bytes!(r#"../libs\x64/WindowsCodecs.lib"#);
static X64_WINFAX: &[u8] = include_bytes!(r#"../libs\x64/WINFAX.lib"#);
static X64_WINHTTP: &[u8] = include_bytes!(r#"../libs\x64/WINHTTP.lib"#);
static X64_WINHVEMULATION: &[u8] = include_bytes!(r#"../libs\x64/WinHvEmulation.lib"#);
static X64_WINHVPLATFORM: &[u8] = include_bytes!(r#"../libs\x64/WinHvPlatform.lib"#);
static X64_WININET: &[u8] = include_bytes!(r#"../libs\x64/WININET.lib"#);
static X64_WINML: &[u8] = include_bytes!(r#"../libs\x64/winml.lib"#);
static X64_WINMM: &[u8] = include_bytes!(r#"../libs\x64/WINMM.lib"#);
static X64_WINSCARD: &[u8] = include_bytes!(r#"../libs\x64/WinSCard.lib"#);
static X64_WINSPOOL: &[u8] = include_bytes!(r#"../libs\x64/WINSPOOL.lib"#);
static X64_WINTRUST: &[u8] = include_bytes!(r#"../libs\x64/WINTRUST.lib"#);
static X64_WINUSB: &[u8] = include_bytes!(r#"../libs\x64/WINUSB.lib"#);
static X64_WLANAPI: &[u8] = include_bytes!(r#"../libs\x64/wlanapi.lib"#);
static X64_WLANUI: &[u8] = include_bytes!(r#"../libs\x64/wlanui.lib"#);
static X64_WLDAP32: &[u8] = include_bytes!(r#"../libs\x64/WLDAP32.lib"#);
static X64_WLDP: &[u8] = include_bytes!(r#"../libs\x64/Wldp.lib"#);
static X64_WMVCORE: &[u8] = include_bytes!(r#"../libs\x64/WMVCore.lib"#);
static X64_WNVAPI: &[u8] = include_bytes!(r#"../libs\x64/wnvapi.lib"#);
static X64_WOFUTIL: &[u8] = include_bytes!(r#"../libs\x64/WOFUTIL.lib"#);
static X64_WS2_32: &[u8] = include_bytes!(r#"../libs\x64/WS2_32.lib"#);
static X64_WSCAPI: &[u8] = include_bytes!(r#"../libs\x64/WSCAPI.lib"#);
static X64_WSCLIENT: &[u8] = include_bytes!(r#"../libs\x64/WSClient.lib"#);
static X64_WSDAPI: &[u8] = include_bytes!(r#"../libs\x64/wsdapi.lib"#);
static X64_WSMSVC: &[u8] = include_bytes!(r#"../libs\x64/WsmSvc.lib"#);
static X64_WSNMP32: &[u8] = include_bytes!(r#"../libs\x64/wsnmp32.lib"#);
static X64_WTSAPI32: &[u8] = include_bytes!(r#"../libs\x64/WTSAPI32.lib"#);
static X64_XAUDIO2_8: &[u8] = include_bytes!(r#"../libs\x64/XAudio2_8.lib"#);
static X64_XINPUTUAP: &[u8] = include_bytes!(r#"../libs\x64/XINPUTUAP.lib"#);
static X64_XMLLITE: &[u8] = include_bytes!(r#"../libs\x64/XmlLite.lib"#);
static X86_ACLUI: &[u8] = include_bytes!(r#"../libs\x86/ACLUI.lib"#);
static X86_ACTIVEDS: &[u8] = include_bytes!(r#"../libs\x86/ACTIVEDS.lib"#);
static X86_ADVAPI32: &[u8] = include_bytes!(r#"../libs\x86/ADVAPI32.lib"#);
static X86_ADVPACK: &[u8] = include_bytes!(r#"../libs\x86/ADVPACK.lib"#);
static X86_AMSI: &[u8] = include_bytes!(r#"../libs\x86/Amsi.lib"#);
static X86_API_MS_WIN_APPMODEL_RUNTIME_L1_1_1: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-appmodel-runtime-l1-1-1.lib"#);
static X86_API_MS_WIN_APPMODEL_RUNTIME_L1_1_3: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-appmodel-runtime-l1-1-3.lib"#);
static X86_API_MS_WIN_CORE_APIQUERY_L2_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-apiquery-l2-1-0.lib"#);
static X86_API_MS_WIN_CORE_BACKGROUNDTASK_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-backgroundtask-l1-1-0.lib"#);
static X86_API_MS_WIN_CORE_COMM_L1_1_1: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-comm-l1-1-1.lib"#);
static X86_API_MS_WIN_CORE_COMM_L1_1_2: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-comm-l1-1-2.lib"#);
static X86_API_MS_WIN_CORE_ERRORHANDLING_L1_1_3: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-errorhandling-l1-1-3.lib"#);
static X86_API_MS_WIN_CORE_FEATURESTAGING_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-featurestaging-l1-1-0.lib"#);
static X86_API_MS_WIN_CORE_FEATURESTAGING_L1_1_1: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-featurestaging-l1-1-1.lib"#);
static X86_API_MS_WIN_CORE_FILE_FROMAPP_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-file-fromapp-l1-1-0.lib"#);
static X86_API_MS_WIN_CORE_HANDLE_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-handle-l1-1-0.lib"#);
static X86_API_MS_WIN_CORE_LIBRARYLOADER_L2_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-libraryloader-l2-1-0.lib"#);
static X86_API_MS_WIN_CORE_MEMORY_L1_1_3: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-memory-l1-1-3.lib"#);
static X86_API_MS_WIN_CORE_MEMORY_L1_1_4: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-memory-l1-1-4.lib"#);
static X86_API_MS_WIN_CORE_MEMORY_L1_1_5: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-memory-l1-1-5.lib"#);
static X86_API_MS_WIN_CORE_MEMORY_L1_1_6: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-memory-l1-1-6.lib"#);
static X86_API_MS_WIN_CORE_MEMORY_L1_1_7: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-memory-l1-1-7.lib"#);
static X86_API_MS_WIN_CORE_PATH_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-path-l1-1-0.lib"#);
static X86_API_MS_WIN_CORE_PSM_APPNOTIFY_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-psm-appnotify-l1-1-0.lib"#);
static X86_API_MS_WIN_CORE_PSM_APPNOTIFY_L1_1_1: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-psm-appnotify-l1-1-1.lib"#);
static X86_API_MS_WIN_CORE_REALTIME_L1_1_1: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-realtime-l1-1-1.lib"#);
static X86_API_MS_WIN_CORE_REALTIME_L1_1_2: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-realtime-l1-1-2.lib"#);
static X86_API_MS_WIN_CORE_SLAPI_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-slapi-l1-1-0.lib"#);
static X86_API_MS_WIN_CORE_STATE_HELPERS_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-state-helpers-l1-1-0.lib"#);
static X86_API_MS_WIN_CORE_SYSINFO_L1_2_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-sysinfo-l1-2-0.lib"#);
static X86_API_MS_WIN_CORE_SYSINFO_L1_2_3: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-sysinfo-l1-2-3.lib"#);
static X86_API_MS_WIN_CORE_SYSINFO_L1_2_4: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-sysinfo-l1-2-4.lib"#);
static X86_API_MS_WIN_CORE_UTIL_L1_1_1: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-util-l1-1-1.lib"#);
static X86_API_MS_WIN_CORE_WINRT_ERROR_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-winrt-error-l1-1-0.lib"#);
static X86_API_MS_WIN_CORE_WINRT_ERROR_L1_1_1: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-winrt-error-l1-1-1.lib"#);
static X86_API_MS_WIN_CORE_WINRT_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-winrt-l1-1-0.lib"#);
static X86_API_MS_WIN_CORE_WINRT_REGISTRATION_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-winrt-registration-l1-1-0.lib"#);
static X86_API_MS_WIN_CORE_WINRT_ROBUFFER_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-winrt-robuffer-l1-1-0.lib"#);
static X86_API_MS_WIN_CORE_WINRT_ROPARAMETERIZEDIID_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-winrt-roparameterizediid-l1-1-0.lib"#);
static X86_API_MS_WIN_CORE_WINRT_STRING_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-winrt-string-l1-1-0.lib"#);
static X86_API_MS_WIN_CORE_WINRT_STRING_L1_1_1: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-winrt-string-l1-1-1.lib"#);
static X86_API_MS_WIN_CORE_WOW64_L1_1_1: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-core-wow64-l1-1-1.lib"#);
static X86_API_MS_WIN_DEVICES_QUERY_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-devices-query-l1-1-0.lib"#);
static X86_API_MS_WIN_DEVICES_QUERY_L1_1_1: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-devices-query-l1-1-1.lib"#);
static X86_API_MS_WIN_DX_D3DKMT_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-dx-d3dkmt-l1-1-0.lib"#);
static X86_API_MS_WIN_GAMING_DEVICEINFORMATION_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-gaming-deviceinformation-l1-1-0.lib"#);
static X86_API_MS_WIN_GAMING_EXPANDEDRESOURCES_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-gaming-expandedresources-l1-1-0.lib"#);
static X86_API_MS_WIN_GAMING_TCUI_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-gaming-tcui-l1-1-0.lib"#);
static X86_API_MS_WIN_GAMING_TCUI_L1_1_1: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-gaming-tcui-l1-1-1.lib"#);
static X86_API_MS_WIN_GAMING_TCUI_L1_1_2: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-gaming-tcui-l1-1-2.lib"#);
static X86_API_MS_WIN_GAMING_TCUI_L1_1_3: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-gaming-tcui-l1-1-3.lib"#);
static X86_API_MS_WIN_GAMING_TCUI_L1_1_4: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-gaming-tcui-l1-1-4.lib"#);
static X86_API_MS_WIN_MM_MISC_L1_1_1: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-mm-misc-l1-1-1.lib"#);
static X86_API_MS_WIN_NET_ISOLATION_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-net-isolation-l1-1-0.lib"#);
static X86_API_MS_WIN_SECURITY_BASE_L1_2_2: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-security-base-l1-2-2.lib"#);
static X86_API_MS_WIN_SECURITY_ISOLATEDCONTAINER_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-security-isolatedcontainer-l1-1-0.lib"#);
static X86_API_MS_WIN_SECURITY_ISOLATEDCONTAINER_L1_1_1: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-security-isolatedcontainer-l1-1-1.lib"#);
static X86_API_MS_WIN_SERVICE_CORE_L1_1_3: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-service-core-l1-1-3.lib"#);
static X86_API_MS_WIN_SERVICE_CORE_L1_1_4: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-service-core-l1-1-4.lib"#);
static X86_API_MS_WIN_SHCORE_SCALING_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-shcore-scaling-l1-1-0.lib"#);
static X86_API_MS_WIN_SHCORE_SCALING_L1_1_1: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-shcore-scaling-l1-1-1.lib"#);
static X86_API_MS_WIN_SHCORE_SCALING_L1_1_2: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-shcore-scaling-l1-1-2.lib"#);
static X86_API_MS_WIN_SHCORE_STREAM_WINRT_L1_1_0: &[u8] = include_bytes!(r#"../libs\x86/api-ms-win-shcore-stream-winrt-l1-1-0.lib"#);
static X86_APPHELP: &[u8] = include_bytes!(r#"../libs\x86/APPHELP.lib"#);
static X86_AUTHZ: &[u8] = include_bytes!(r#"../libs\x86/AUTHZ.lib"#);
static X86_AVICAP32: &[u8] = include_bytes!(r#"../libs\x86/AVICAP32.lib"#);
static X86_AVIFIL32: &[u8] = include_bytes!(r#"../libs\x86/AVIFIL32.lib"#);
static X86_BCRYPT: &[u8] = include_bytes!(r#"../libs\x86/bcrypt.lib"#);
static X86_BLUETOOTHAPIS: &[u8] = include_bytes!(r#"../libs\x86/BluetoothApis.lib"#);
static X86_BTHPROPS: &[u8] = include_bytes!(r#"../libs\x86/bthprops.lib"#);
static X86_CABINET: &[u8] = include_bytes!(r#"../libs\x86/Cabinet.lib"#);
static X86_CERTADM: &[u8] = include_bytes!(r#"../libs\x86/certadm.lib"#);
static X86_CERTPOLENG: &[u8] = include_bytes!(r#"../libs\x86/certpoleng.lib"#);
static X86_CFGMGR32: &[u8] = include_bytes!(r#"../libs\x86/CFGMGR32.lib"#);
static X86_CHAKRA: &[u8] = include_bytes!(r#"../libs\x86/chakra.lib"#);
static X86_CLDAPI: &[u8] = include_bytes!(r#"../libs\x86/cldapi.lib"#);
static X86_CLFSW32: &[u8] = include_bytes!(r#"../libs\x86/clfsw32.lib"#);
static X86_CLUSAPI: &[u8] = include_bytes!(r#"../libs\x86/CLUSAPI.lib"#);
static X86_COMCTL32: &[u8] = include_bytes!(r#"../libs\x86/COMCTL32.lib"#);
static X86_COMDLG32: &[u8] = include_bytes!(r#"../libs\x86/COMDLG32.lib"#);
static X86_COMSVCS: &[u8] = include_bytes!(r#"../libs\x86/comsvcs.lib"#);
static X86_COREMESSAGING: &[u8] = include_bytes!(r#"../libs\x86/CoreMessaging.lib"#);
static X86_CREDUI: &[u8] = include_bytes!(r#"../libs\x86/credui.lib"#);
static X86_CRYPT32: &[u8] = include_bytes!(r#"../libs\x86/CRYPT32.lib"#);
static X86_CRYPTNET: &[u8] = include_bytes!(r#"../libs\x86/CRYPTNET.lib"#);
static X86_CRYPTUI: &[u8] = include_bytes!(r#"../libs\x86/CRYPTUI.lib"#);
static X86_CRYPTXML: &[u8] = include_bytes!(r#"../libs\x86/CRYPTXML.lib"#);
static X86_CSCAPI: &[u8] = include_bytes!(r#"../libs\x86/CSCAPI.lib"#);
static X86_D2D1: &[u8] = include_bytes!(r#"../libs\x86/d2d1.lib"#);
static X86_D3D10: &[u8] = include_bytes!(r#"../libs\x86/d3d10.lib"#);
static X86_D3D10_1: &[u8] = include_bytes!(r#"../libs\x86/d3d10_1.lib"#);
static X86_D3D11: &[u8] = include_bytes!(r#"../libs\x86/d3d11.lib"#);
static X86_D3D12: &[u8] = include_bytes!(r#"../libs\x86/d3d12.lib"#);
static X86_D3D9: &[u8] = include_bytes!(r#"../libs\x86/d3d9.lib"#);
static X86_D3DCOMPILER_47: &[u8] = include_bytes!(r#"../libs\x86/D3DCOMPILER_47.lib"#);
static X86_D3DCSX: &[u8] = include_bytes!(r#"../libs\x86/d3dcsx.lib"#);
static X86_DAVCLNT: &[u8] = include_bytes!(r#"../libs\x86/davclnt.lib"#);
static X86_DBGENG: &[u8] = include_bytes!(r#"../libs\x86/dbgeng.lib"#);
static X86_DBGHELP: &[u8] = include_bytes!(r#"../libs\x86/dbghelp.lib"#);
static X86_DBGMODEL: &[u8] = include_bytes!(r#"../libs\x86/dbgmodel.lib"#);
static X86_DCIMAN32: &[u8] = include_bytes!(r#"../libs\x86/DCIMAN32.lib"#);
static X86_DCOMP: &[u8] = include_bytes!(r#"../libs\x86/dcomp.lib"#);
static X86_DDRAW: &[u8] = include_bytes!(r#"../libs\x86/DDRAW.lib"#);
static X86_DEVICEACCESS: &[u8] = include_bytes!(r#"../libs\x86/deviceaccess.lib"#);
static X86_DFLAYOUT: &[u8] = include_bytes!(r#"../libs\x86/dflayout.lib"#);
static X86_DHCPCSVC: &[u8] = include_bytes!(r#"../libs\x86/dhcpcsvc.lib"#);
static X86_DHCPCSVC6: &[u8] = include_bytes!(r#"../libs\x86/dhcpcsvc6.lib"#);
static X86_DHCPSAPI: &[u8] = include_bytes!(r#"../libs\x86/DHCPSAPI.lib"#);
static X86_DIAGNOSTICDATAQUERY: &[u8] = include_bytes!(r#"../libs\x86/DiagnosticDataQuery.lib"#);
static X86_DINPUT8: &[u8] = include_bytes!(r#"../libs\x86/DINPUT8.lib"#);
static X86_DIRECTML: &[u8] = include_bytes!(r#"../libs\x86/DirectML.lib"#);
static X86_DNSAPI: &[u8] = include_bytes!(r#"../libs\x86/DNSAPI.lib"#);
static X86_DRT: &[u8] = include_bytes!(r#"../libs\x86/drt.lib"#);
static X86_DRTPROV: &[u8] = include_bytes!(r#"../libs\x86/drtprov.lib"#);
static X86_DRTTRANSPORT: &[u8] = include_bytes!(r#"../libs\x86/drttransport.lib"#);
static X86_DSOUND: &[u8] = include_bytes!(r#"../libs\x86/DSOUND.lib"#);
static X86_DSPARSE: &[u8] = include_bytes!(r#"../libs\x86/DSPARSE.lib"#);
static X86_DSPROP: &[u8] = include_bytes!(r#"../libs\x86/dsprop.lib"#);
static X86_DSSEC: &[u8] = include_bytes!(r#"../libs\x86/DSSEC.lib"#);
static X86_DSUIEXT: &[u8] = include_bytes!(r#"../libs\x86/dsuiext.lib"#);
static X86_DWMAPI: &[u8] = include_bytes!(r#"../libs\x86/dwmapi.lib"#);
static X86_DWRITE: &[u8] = include_bytes!(r#"../libs\x86/DWrite.lib"#);
static X86_DXGI: &[u8] = include_bytes!(r#"../libs\x86/dxgi.lib"#);
static X86_DXVA2: &[u8] = include_bytes!(r#"../libs\x86/dxva2.lib"#);
static X86_EAPPCFG: &[u8] = include_bytes!(r#"../libs\x86/eappcfg.lib"#);
static X86_EAPPPRXY: &[u8] = include_bytes!(r#"../libs\x86/eappprxy.lib"#);
static X86_EFSWRT: &[u8] = include_bytes!(r#"../libs\x86/efswrt.lib"#);
static X86_ELSCORE: &[u8] = include_bytes!(r#"../libs\x86/elscore.lib"#);
static X86_ESENT: &[u8] = include_bytes!(r#"../libs\x86/ESENT.lib"#);
static X86_EVR: &[u8] = include_bytes!(r#"../libs\x86/EVR.lib"#);
static X86_FAULTREP: &[u8] = include_bytes!(r#"../libs\x86/faultrep.lib"#);
static X86_FHSVCCTL: &[u8] = include_bytes!(r#"../libs\x86/fhsvcctl.lib"#);
static X86_FLTLIB: &[u8] = include_bytes!(r#"../libs\x86/FLTLIB.lib"#);
static X86_FONTSUB: &[u8] = include_bytes!(r#"../libs\x86/FONTSUB.lib"#);
static X86_FWPUCLNT: &[u8] = include_bytes!(r#"../libs\x86/fwpuclnt.lib"#);
static X86_FXSUTILITY: &[u8] = include_bytes!(r#"../libs\x86/fxsutility.lib"#);
static X86_GDI32: &[u8] = include_bytes!(r#"../libs\x86/GDI32.lib"#);
static X86_GPEDIT: &[u8] = include_bytes!(r#"../libs\x86/GPEDIT.lib"#);
static X86_HID: &[u8] = include_bytes!(r#"../libs\x86/HID.lib"#);
static X86_HLINK: &[u8] = include_bytes!(r#"../libs\x86/hlink.lib"#);
static X86_HRTFAPO: &[u8] = include_bytes!(r#"../libs\x86/HrtfApo.lib"#);
static X86_HTTPAPI: &[u8] = include_bytes!(r#"../libs\x86/HTTPAPI.lib"#);
static X86_ICM32: &[u8] = include_bytes!(r#"../libs\x86/ICM32.lib"#);
static X86_ICMUI: &[u8] = include_bytes!(r#"../libs\x86/ICMUI.lib"#);
static X86_ICU: &[u8] = include_bytes!(r#"../libs\x86/icu.lib"#);
static X86_IMM32: &[u8] = include_bytes!(r#"../libs\x86/IMM32.lib"#);
static X86_INKOBJCORE: &[u8] = include_bytes!(r#"../libs\x86/inkobjcore.lib"#);
static X86_IPHLPAPI: &[u8] = include_bytes!(r#"../libs\x86/IPHLPAPI.lib"#);
static X86_ISCSIDSC: &[u8] = include_bytes!(r#"../libs\x86/ISCSIDSC.lib"#);
static X86_KERNEL32: &[u8] = include_bytes!(r#"../libs\x86/KERNEL32.lib"#);
static X86_KEYCREDMGR: &[u8] = include_bytes!(r#"../libs\x86/KeyCredMgr.lib"#);
static X86_KSUSER: &[u8] = include_bytes!(r#"../libs\x86/ksuser.lib"#);
static X86_KTMW32: &[u8] = include_bytes!(r#"../libs\x86/ktmw32.lib"#);
static X86_LOADPERF: &[u8] = include_bytes!(r#"../libs\x86/loadperf.lib"#);
static X86_MAGNIFICATION: &[u8] = include_bytes!(r#"../libs\x86/MAGNIFICATION.lib"#);
static X86_MAPI32: &[u8] = include_bytes!(r#"../libs\x86/MAPI32.lib"#);
static X86_MDMREGISTRATION: &[u8] = include_bytes!(r#"../libs\x86/MDMRegistration.lib"#);
static X86_MF: &[u8] = include_bytes!(r#"../libs\x86/MF.lib"#);
static X86_MFCORE: &[u8] = include_bytes!(r#"../libs\x86/MFCORE.lib"#);
static X86_MFPLAT: &[u8] = include_bytes!(r#"../libs\x86/MFPlat.lib"#);
static X86_MFPLAY: &[u8] = include_bytes!(r#"../libs\x86/MFPlay.lib"#);
static X86_MFREADWRITE: &[u8] = include_bytes!(r#"../libs\x86/MFReadWrite.lib"#);
static X86_MFSENSORGROUP: &[u8] = include_bytes!(r#"../libs\x86/MFSENSORGROUP.lib"#);
static X86_MFSRCSNK: &[u8] = include_bytes!(r#"../libs\x86/mfsrcsnk.lib"#);
static X86_MGMTAPI: &[u8] = include_bytes!(r#"../libs\x86/mgmtapi.lib"#);
static X86_MI: &[u8] = include_bytes!(r#"../libs\x86/mi.lib"#);
static X86_MMDEVAPI: &[u8] = include_bytes!(r#"../libs\x86/MMDevAPI.lib"#);
static X86_MPR: &[u8] = include_bytes!(r#"../libs\x86/MPR.lib"#);
static X86_MPRAPI: &[u8] = include_bytes!(r#"../libs\x86/MPRAPI.lib"#);
static X86_MRMSUPPORT: &[u8] = include_bytes!(r#"../libs\x86/MrmSupport.lib"#);
static X86_MSACM32: &[u8] = include_bytes!(r#"../libs\x86/MSACM32.lib"#);
static X86_MSAJAPI: &[u8] = include_bytes!(r#"../libs\x86/MSAJApi.lib"#);
static X86_MSCMS: &[u8] = include_bytes!(r#"../libs\x86/mscms.lib"#);
static X86_MSCTFMONITOR: &[u8] = include_bytes!(r#"../libs\x86/MsCtfMonitor.lib"#);
static X86_MSDMO: &[u8] = include_bytes!(r#"../libs\x86/msdmo.lib"#);
static X86_MSDRM: &[u8] = include_bytes!(r#"../libs\x86/msdrm.lib"#);
static X86_MSI: &[u8] = include_bytes!(r#"../libs\x86/msi.lib"#);
static X86_MSIMG32: &[u8] = include_bytes!(r#"../libs\x86/MSIMG32.lib"#);
static X86_MSPORTS: &[u8] = include_bytes!(r#"../libs\x86/MSPORTS.lib"#);
static X86_MSTASK: &[u8] = include_bytes!(r#"../libs\x86/mstask.lib"#);
static X86_MSVFW32: &[u8] = include_bytes!(r#"../libs\x86/MSVFW32.lib"#);
static X86_MSWSOCK: &[u8] = include_bytes!(r#"../libs\x86/MSWSOCK.lib"#);
static X86_MTXDM: &[u8] = include_bytes!(r#"../libs\x86/MTxDM.lib"#);
static X86_NCRYPT: &[u8] = include_bytes!(r#"../libs\x86/ncrypt.lib"#);
static X86_NDFAPI: &[u8] = include_bytes!(r#"../libs\x86/NDFAPI.lib"#);
static X86_NETAPI32: &[u8] = include_bytes!(r#"../libs\x86/NETAPI32.lib"#);
static X86_NETSH: &[u8] = include_bytes!(r#"../libs\x86/NETSH.lib"#);
static X86_NEWDEV: &[u8] = include_bytes!(r#"../libs\x86/newdev.lib"#);
static X86_NINPUT: &[u8] = include_bytes!(r#"../libs\x86/NInput.lib"#);
static X86_NORMALIZ: &[u8] = include_bytes!(r#"../libs\x86/NORMALIZ.lib"#);
static X86_NTDLL: &[u8] = include_bytes!(r#"../libs\x86/ntdll.lib"#);
static X86_NTDSAPI: &[u8] = include_bytes!(r#"../libs\x86/NTDSAPI.lib"#);
static X86_NTLANMAN: &[u8] = include_bytes!(r#"../libs\x86/NTLANMAN.lib"#);
static X86_OLE32: &[u8] = include_bytes!(r#"../libs\x86/OLE32.lib"#);
static X86_OLEACC: &[u8] = include_bytes!(r#"../libs\x86/OLEACC.lib"#);
static X86_OLEAUT32: &[u8] = include_bytes!(r#"../libs\x86/OLEAUT32.lib"#);
static X86_OLEDLG: &[u8] = include_bytes!(r#"../libs\x86/oledlg.lib"#);
static X86_ONDEMANDCONNROUTEHELPER: &[u8] = include_bytes!(r#"../libs\x86/OnDemandConnRouteHelper.lib"#);
static X86_OPENGL32: &[u8] = include_bytes!(r#"../libs\x86/OPENGL32.lib"#);
static X86_P2P: &[u8] = include_bytes!(r#"../libs\x86/P2P.lib"#);
static X86_P2PGRAPH: &[u8] = include_bytes!(r#"../libs\x86/P2PGRAPH.lib"#);
static X86_PDH: &[u8] = include_bytes!(r#"../libs\x86/pdh.lib"#);
static X86_PEERDIST: &[u8] = include_bytes!(r#"../libs\x86/PeerDist.lib"#);
static X86_POWRPROF: &[u8] = include_bytes!(r#"../libs\x86/POWRPROF.lib"#);
static X86_PRNTVPT: &[u8] = include_bytes!(r#"../libs\x86/prntvpt.lib"#);
static X86_PROJECTEDFSLIB: &[u8] = include_bytes!(r#"../libs\x86/PROJECTEDFSLIB.lib"#);
static X86_PROPSYS: &[u8] = include_bytes!(r#"../libs\x86/PROPSYS.lib"#);
static X86_QUARTZ: &[u8] = include_bytes!(r#"../libs\x86/QUARTZ.lib"#);
static X86_QUERY: &[u8] = include_bytes!(r#"../libs\x86/query.lib"#);
static X86_QWAVE: &[u8] = include_bytes!(r#"../libs\x86/qwave.lib"#);
static X86_RASAPI32: &[u8] = include_bytes!(r#"../libs\x86/RASAPI32.lib"#);
static X86_RASDLG: &[u8] = include_bytes!(r#"../libs\x86/RASDLG.lib"#);
static X86_RESUTILS: &[u8] = include_bytes!(r#"../libs\x86/RESUTILS.lib"#);
static X86_ROMETADATA: &[u8] = include_bytes!(r#"../libs\x86/RoMetadata.lib"#);
static X86_RPCNS4: &[u8] = include_bytes!(r#"../libs\x86/RPCNS4.lib"#);
static X86_RPCRT4: &[u8] = include_bytes!(r#"../libs\x86/RPCRT4.lib"#);
static X86_RSTRTMGR: &[u8] = include_bytes!(r#"../libs\x86/RstrtMgr.lib"#);
static X86_RTM: &[u8] = include_bytes!(r#"../libs\x86/rtm.lib"#);
static X86_SAS: &[u8] = include_bytes!(r#"../libs\x86/SAS.lib"#);
static X86_SCARDDLG: &[u8] = include_bytes!(r#"../libs\x86/SCARDDLG.lib"#);
static X86_SCHANNEL: &[u8] = include_bytes!(r#"../libs\x86/SCHANNEL.lib"#);
static X86_SECUR32: &[u8] = include_bytes!(r#"../libs\x86/SECUR32.lib"#);
static X86_SENSAPI: &[u8] = include_bytes!(r#"../libs\x86/SensApi.lib"#);
static X86_SETUPAPI: &[u8] = include_bytes!(r#"../libs\x86/SETUPAPI.lib"#);
static X86_SFC: &[u8] = include_bytes!(r#"../libs\x86/sfc.lib"#);
static X86_SHDOCVW: &[u8] = include_bytes!(r#"../libs\x86/SHDOCVW.lib"#);
static X86_SHELL32: &[u8] = include_bytes!(r#"../libs\x86/SHELL32.lib"#);
static X86_SHLWAPI: &[u8] = include_bytes!(r#"../libs\x86/SHLWAPI.lib"#);
static X86_SLC: &[u8] = include_bytes!(r#"../libs\x86/SLC.lib"#);
static X86_SLCEXT: &[u8] = include_bytes!(r#"../libs\x86/slcext.lib"#);
static X86_SLWGA: &[u8] = include_bytes!(r#"../libs\x86/SLWGA.lib"#);
static X86_SNMPAPI: &[u8] = include_bytes!(r#"../libs\x86/snmpapi.lib"#);
static X86_SRPAPI: &[u8] = include_bytes!(r#"../libs\x86/srpapi.lib"#);
static X86_SSPICLI: &[u8] = include_bytes!(r#"../libs\x86/SspiCli.lib"#);
static X86_T2EMBED: &[u8] = include_bytes!(r#"../libs\x86/t2embed.lib"#);
static X86_TAPI32: &[u8] = include_bytes!(r#"../libs\x86/TAPI32.lib"#);
static X86_TBS: &[u8] = include_bytes!(r#"../libs\x86/tbs.lib"#);
static X86_TDH: &[u8] = include_bytes!(r#"../libs\x86/TDH.lib"#);
static X86_TOKENBINDING: &[u8] = include_bytes!(r#"../libs\x86/TOKENBINDING.lib"#);
static X86_TRAFFIC: &[u8] = include_bytes!(r#"../libs\x86/TRAFFIC.lib"#);
static X86_TXFW32: &[u8] = include_bytes!(r#"../libs\x86/txfw32.lib"#);
static X86_UALAPI: &[u8] = include_bytes!(r#"../libs\x86/ualapi.lib"#);
static X86_UIAUTOMATIONCORE: &[u8] = include_bytes!(r#"../libs\x86/UIAutomationCore.lib"#);
static X86_URLMON: &[u8] = include_bytes!(r#"../libs\x86/URLMON.lib"#);
static X86_USER32: &[u8] = include_bytes!(r#"../libs\x86/USER32.lib"#);
static X86_USERENV: &[u8] = include_bytes!(r#"../libs\x86/USERENV.lib"#);
static X86_USP10: &[u8] = include_bytes!(r#"../libs\x86/USP10.lib"#);
static X86_UXTHEME: &[u8] = include_bytes!(r#"../libs\x86/UXTHEME.lib"#);
static X86_VERIFIER: &[u8] = include_bytes!(r#"../libs\x86/verifier.lib"#);
static X86_VERSION: &[u8] = include_bytes!(r#"../libs\x86/VERSION.lib"#);
static X86_VERTDLL: &[u8] = include_bytes!(r#"../libs\x86/vertdll.lib"#);
static X86_VIRTDISK: &[u8] = include_bytes!(r#"../libs\x86/VirtDisk.lib"#);
static X86_VSSAPI: &[u8] = include_bytes!(r#"../libs\x86/VSSAPI.lib"#);
static X86_WCMAPI: &[u8] = include_bytes!(r#"../libs\x86/wcmapi.lib"#);
static X86_WDSBP: &[u8] = include_bytes!(r#"../libs\x86/WDSBP.lib"#);
static X86_WDSCLIENTAPI: &[u8] = include_bytes!(r#"../libs\x86/WDSCLIENTAPI.lib"#);
static X86_WDSMC: &[u8] = include_bytes!(r#"../libs\x86/WDSMC.lib"#);
static X86_WDSPXE: &[u8] = include_bytes!(r#"../libs\x86/WDSPXE.lib"#);
static X86_WDSTPTC: &[u8] = include_bytes!(r#"../libs\x86/WDSTPTC.lib"#);
static X86_WEBAUTHN: &[u8] = include_bytes!(r#"../libs\x86/webauthn.lib"#);
static X86_WEBSERVICES: &[u8] = include_bytes!(r#"../libs\x86/webservices.lib"#);
static X86_WEBSOCKET: &[u8] = include_bytes!(r#"../libs\x86/websocket.lib"#);
static X86_WECAPI: &[u8] = include_bytes!(r#"../libs\x86/WecApi.lib"#);
static X86_WER: &[u8] = include_bytes!(r#"../libs\x86/wer.lib"#);
static X86_WEVTAPI: &[u8] = include_bytes!(r#"../libs\x86/wevtapi.lib"#);
static X86_WINBIO: &[u8] = include_bytes!(r#"../libs\x86/winbio.lib"#);
static X86_WINDOWS_AI_MACHINELEARNING: &[u8] = include_bytes!(r#"../libs\x86/windows.ai.machinelearning.lib"#);
static X86_WINDOWS_DATA_PDF: &[u8] = include_bytes!(r#"../libs\x86/Windows.Data.Pdf.lib"#);
static X86_WINDOWS_NETWORKING: &[u8] = include_bytes!(r#"../libs\x86/Windows.Networking.lib"#);
static X86_WINDOWS_UI_XAML: &[u8] = include_bytes!(r#"../libs\x86/Windows.UI.Xaml.lib"#);
static X86_WINDOWSCODECS: &[u8] = include_bytes!(r#"../libs\x86/WindowsCodecs.lib"#);
static X86_WINFAX: &[u8] = include_bytes!(r#"../libs\x86/WINFAX.lib"#);
static X86_WINHTTP: &[u8] = include_bytes!(r#"../libs\x86/WINHTTP.lib"#);
static X86_WINHVEMULATION: &[u8] = include_bytes!(r#"../libs\x86/WinHvEmulation.lib"#);
static X86_WINHVPLATFORM: &[u8] = include_bytes!(r#"../libs\x86/WinHvPlatform.lib"#);
static X86_WININET: &[u8] = include_bytes!(r#"../libs\x86/WININET.lib"#);
static X86_WINML: &[u8] = include_bytes!(r#"../libs\x86/winml.lib"#);
static X86_WINMM: &[u8] = include_bytes!(r#"../libs\x86/WINMM.lib"#);
static X86_WINSCARD: &[u8] = include_bytes!(r#"../libs\x86/WinSCard.lib"#);
static X86_WINSPOOL: &[u8] = include_bytes!(r#"../libs\x86/WINSPOOL.lib"#);
static X86_WINTRUST: &[u8] = include_bytes!(r#"../libs\x86/WINTRUST.lib"#);
static X86_WINUSB: &[u8] = include_bytes!(r#"../libs\x86/WINUSB.lib"#);
static X86_WLANAPI: &[u8] = include_bytes!(r#"../libs\x86/wlanapi.lib"#);
static X86_WLANUI: &[u8] = include_bytes!(r#"../libs\x86/wlanui.lib"#);
static X86_WLDAP32: &[u8] = include_bytes!(r#"../libs\x86/WLDAP32.lib"#);
static X86_WLDP: &[u8] = include_bytes!(r#"../libs\x86/Wldp.lib"#);
static X86_WMVCORE: &[u8] = include_bytes!(r#"../libs\x86/WMVCore.lib"#);
static X86_WNVAPI: &[u8] = include_bytes!(r#"../libs\x86/wnvapi.lib"#);
static X86_WOFUTIL: &[u8] = include_bytes!(r#"../libs\x86/WOFUTIL.lib"#);
static X86_WS2_32: &[u8] = include_bytes!(r#"../libs\x86/WS2_32.lib"#);
static X86_WSCAPI: &[u8] = include_bytes!(r#"../libs\x86/WSCAPI.lib"#);
static X86_WSCLIENT: &[u8] = include_bytes!(r#"../libs\x86/WSClient.lib"#);
static X86_WSDAPI: &[u8] = include_bytes!(r#"../libs\x86/wsdapi.lib"#);
static X86_WSMSVC: &[u8] = include_bytes!(r#"../libs\x86/WsmSvc.lib"#);
static X86_WSNMP32: &[u8] = include_bytes!(r#"../libs\x86/wsnmp32.lib"#);
static X86_WTSAPI32: &[u8] = include_bytes!(r#"../libs\x86/WTSAPI32.lib"#);
static X86_XAUDIO2_8: &[u8] = include_bytes!(r#"../libs\x86/XAudio2_8.lib"#);
static X86_XINPUTUAP: &[u8] = include_bytes!(r#"../libs\x86/XINPUTUAP.lib"#);
static X86_XMLLITE: &[u8] = include_bytes!(r#"../libs\x86/XmlLite.lib"#);
static ARM_ACLUI: &[u8] = include_bytes!(r#"../libs\arm/ACLUI.lib"#);
static ARM_ACTIVEDS: &[u8] = include_bytes!(r#"../libs\arm/ACTIVEDS.lib"#);
static ARM_ADVAPI32: &[u8] = include_bytes!(r#"../libs\arm/ADVAPI32.lib"#);
static ARM_ADVPACK: &[u8] = include_bytes!(r#"../libs\arm/ADVPACK.lib"#);
static ARM_AMSI: &[u8] = include_bytes!(r#"../libs\arm/Amsi.lib"#);
static ARM_API_MS_WIN_APPMODEL_RUNTIME_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-appmodel-runtime-l1-1-1.lib"#);
static ARM_API_MS_WIN_APPMODEL_RUNTIME_L1_1_3: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-appmodel-runtime-l1-1-3.lib"#);
static ARM_API_MS_WIN_CORE_APIQUERY_L2_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-apiquery-l2-1-0.lib"#);
static ARM_API_MS_WIN_CORE_BACKGROUNDTASK_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-backgroundtask-l1-1-0.lib"#);
static ARM_API_MS_WIN_CORE_COMM_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-comm-l1-1-1.lib"#);
static ARM_API_MS_WIN_CORE_COMM_L1_1_2: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-comm-l1-1-2.lib"#);
static ARM_API_MS_WIN_CORE_ERRORHANDLING_L1_1_3: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-errorhandling-l1-1-3.lib"#);
static ARM_API_MS_WIN_CORE_FEATURESTAGING_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-featurestaging-l1-1-0.lib"#);
static ARM_API_MS_WIN_CORE_FEATURESTAGING_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-featurestaging-l1-1-1.lib"#);
static ARM_API_MS_WIN_CORE_FILE_FROMAPP_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-file-fromapp-l1-1-0.lib"#);
static ARM_API_MS_WIN_CORE_HANDLE_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-handle-l1-1-0.lib"#);
static ARM_API_MS_WIN_CORE_LIBRARYLOADER_L2_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-libraryloader-l2-1-0.lib"#);
static ARM_API_MS_WIN_CORE_MEMORY_L1_1_3: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-memory-l1-1-3.lib"#);
static ARM_API_MS_WIN_CORE_MEMORY_L1_1_4: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-memory-l1-1-4.lib"#);
static ARM_API_MS_WIN_CORE_MEMORY_L1_1_5: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-memory-l1-1-5.lib"#);
static ARM_API_MS_WIN_CORE_MEMORY_L1_1_6: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-memory-l1-1-6.lib"#);
static ARM_API_MS_WIN_CORE_MEMORY_L1_1_7: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-memory-l1-1-7.lib"#);
static ARM_API_MS_WIN_CORE_PATH_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-path-l1-1-0.lib"#);
static ARM_API_MS_WIN_CORE_PSM_APPNOTIFY_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-psm-appnotify-l1-1-0.lib"#);
static ARM_API_MS_WIN_CORE_PSM_APPNOTIFY_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-psm-appnotify-l1-1-1.lib"#);
static ARM_API_MS_WIN_CORE_REALTIME_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-realtime-l1-1-1.lib"#);
static ARM_API_MS_WIN_CORE_REALTIME_L1_1_2: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-realtime-l1-1-2.lib"#);
static ARM_API_MS_WIN_CORE_SLAPI_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-slapi-l1-1-0.lib"#);
static ARM_API_MS_WIN_CORE_STATE_HELPERS_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-state-helpers-l1-1-0.lib"#);
static ARM_API_MS_WIN_CORE_SYSINFO_L1_2_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-sysinfo-l1-2-0.lib"#);
static ARM_API_MS_WIN_CORE_SYSINFO_L1_2_3: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-sysinfo-l1-2-3.lib"#);
static ARM_API_MS_WIN_CORE_SYSINFO_L1_2_4: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-sysinfo-l1-2-4.lib"#);
static ARM_API_MS_WIN_CORE_UTIL_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-util-l1-1-1.lib"#);
static ARM_API_MS_WIN_CORE_WINRT_ERROR_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-winrt-error-l1-1-0.lib"#);
static ARM_API_MS_WIN_CORE_WINRT_ERROR_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-winrt-error-l1-1-1.lib"#);
static ARM_API_MS_WIN_CORE_WINRT_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-winrt-l1-1-0.lib"#);
static ARM_API_MS_WIN_CORE_WINRT_REGISTRATION_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-winrt-registration-l1-1-0.lib"#);
static ARM_API_MS_WIN_CORE_WINRT_ROBUFFER_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-winrt-robuffer-l1-1-0.lib"#);
static ARM_API_MS_WIN_CORE_WINRT_ROPARAMETERIZEDIID_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-winrt-roparameterizediid-l1-1-0.lib"#);
static ARM_API_MS_WIN_CORE_WINRT_STRING_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-winrt-string-l1-1-0.lib"#);
static ARM_API_MS_WIN_CORE_WINRT_STRING_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-winrt-string-l1-1-1.lib"#);
static ARM_API_MS_WIN_CORE_WOW64_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-core-wow64-l1-1-1.lib"#);
static ARM_API_MS_WIN_DEVICES_QUERY_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-devices-query-l1-1-0.lib"#);
static ARM_API_MS_WIN_DEVICES_QUERY_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-devices-query-l1-1-1.lib"#);
static ARM_API_MS_WIN_DX_D3DKMT_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-dx-d3dkmt-l1-1-0.lib"#);
static ARM_API_MS_WIN_GAMING_DEVICEINFORMATION_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-gaming-deviceinformation-l1-1-0.lib"#);
static ARM_API_MS_WIN_GAMING_EXPANDEDRESOURCES_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-gaming-expandedresources-l1-1-0.lib"#);
static ARM_API_MS_WIN_GAMING_TCUI_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-gaming-tcui-l1-1-0.lib"#);
static ARM_API_MS_WIN_GAMING_TCUI_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-gaming-tcui-l1-1-1.lib"#);
static ARM_API_MS_WIN_GAMING_TCUI_L1_1_2: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-gaming-tcui-l1-1-2.lib"#);
static ARM_API_MS_WIN_GAMING_TCUI_L1_1_3: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-gaming-tcui-l1-1-3.lib"#);
static ARM_API_MS_WIN_GAMING_TCUI_L1_1_4: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-gaming-tcui-l1-1-4.lib"#);
static ARM_API_MS_WIN_MM_MISC_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-mm-misc-l1-1-1.lib"#);
static ARM_API_MS_WIN_NET_ISOLATION_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-net-isolation-l1-1-0.lib"#);
static ARM_API_MS_WIN_SECURITY_BASE_L1_2_2: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-security-base-l1-2-2.lib"#);
static ARM_API_MS_WIN_SECURITY_ISOLATEDCONTAINER_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-security-isolatedcontainer-l1-1-0.lib"#);
static ARM_API_MS_WIN_SECURITY_ISOLATEDCONTAINER_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-security-isolatedcontainer-l1-1-1.lib"#);
static ARM_API_MS_WIN_SERVICE_CORE_L1_1_3: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-service-core-l1-1-3.lib"#);
static ARM_API_MS_WIN_SERVICE_CORE_L1_1_4: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-service-core-l1-1-4.lib"#);
static ARM_API_MS_WIN_SHCORE_SCALING_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-shcore-scaling-l1-1-0.lib"#);
static ARM_API_MS_WIN_SHCORE_SCALING_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-shcore-scaling-l1-1-1.lib"#);
static ARM_API_MS_WIN_SHCORE_SCALING_L1_1_2: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-shcore-scaling-l1-1-2.lib"#);
static ARM_API_MS_WIN_SHCORE_STREAM_WINRT_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm/api-ms-win-shcore-stream-winrt-l1-1-0.lib"#);
static ARM_APPHELP: &[u8] = include_bytes!(r#"../libs\arm/APPHELP.lib"#);
static ARM_AUTHZ: &[u8] = include_bytes!(r#"../libs\arm/AUTHZ.lib"#);
static ARM_AVICAP32: &[u8] = include_bytes!(r#"../libs\arm/AVICAP32.lib"#);
static ARM_AVIFIL32: &[u8] = include_bytes!(r#"../libs\arm/AVIFIL32.lib"#);
static ARM_BCRYPT: &[u8] = include_bytes!(r#"../libs\arm/bcrypt.lib"#);
static ARM_BLUETOOTHAPIS: &[u8] = include_bytes!(r#"../libs\arm/BluetoothApis.lib"#);
static ARM_BTHPROPS: &[u8] = include_bytes!(r#"../libs\arm/bthprops.lib"#);
static ARM_CABINET: &[u8] = include_bytes!(r#"../libs\arm/Cabinet.lib"#);
static ARM_CERTADM: &[u8] = include_bytes!(r#"../libs\arm/certadm.lib"#);
static ARM_CERTPOLENG: &[u8] = include_bytes!(r#"../libs\arm/certpoleng.lib"#);
static ARM_CFGMGR32: &[u8] = include_bytes!(r#"../libs\arm/CFGMGR32.lib"#);
static ARM_CHAKRA: &[u8] = include_bytes!(r#"../libs\arm/chakra.lib"#);
static ARM_CLDAPI: &[u8] = include_bytes!(r#"../libs\arm/cldapi.lib"#);
static ARM_CLFSW32: &[u8] = include_bytes!(r#"../libs\arm/clfsw32.lib"#);
static ARM_CLUSAPI: &[u8] = include_bytes!(r#"../libs\arm/CLUSAPI.lib"#);
static ARM_COMCTL32: &[u8] = include_bytes!(r#"../libs\arm/COMCTL32.lib"#);
static ARM_COMDLG32: &[u8] = include_bytes!(r#"../libs\arm/COMDLG32.lib"#);
static ARM_COMSVCS: &[u8] = include_bytes!(r#"../libs\arm/comsvcs.lib"#);
static ARM_COREMESSAGING: &[u8] = include_bytes!(r#"../libs\arm/CoreMessaging.lib"#);
static ARM_CREDUI: &[u8] = include_bytes!(r#"../libs\arm/credui.lib"#);
static ARM_CRYPT32: &[u8] = include_bytes!(r#"../libs\arm/CRYPT32.lib"#);
static ARM_CRYPTNET: &[u8] = include_bytes!(r#"../libs\arm/CRYPTNET.lib"#);
static ARM_CRYPTUI: &[u8] = include_bytes!(r#"../libs\arm/CRYPTUI.lib"#);
static ARM_CRYPTXML: &[u8] = include_bytes!(r#"../libs\arm/CRYPTXML.lib"#);
static ARM_CSCAPI: &[u8] = include_bytes!(r#"../libs\arm/CSCAPI.lib"#);
static ARM_D2D1: &[u8] = include_bytes!(r#"../libs\arm/d2d1.lib"#);
static ARM_D3D10: &[u8] = include_bytes!(r#"../libs\arm/d3d10.lib"#);
static ARM_D3D10_1: &[u8] = include_bytes!(r#"../libs\arm/d3d10_1.lib"#);
static ARM_D3D11: &[u8] = include_bytes!(r#"../libs\arm/d3d11.lib"#);
static ARM_D3D12: &[u8] = include_bytes!(r#"../libs\arm/d3d12.lib"#);
static ARM_D3D9: &[u8] = include_bytes!(r#"../libs\arm/d3d9.lib"#);
static ARM_D3DCOMPILER_47: &[u8] = include_bytes!(r#"../libs\arm/D3DCOMPILER_47.lib"#);
static ARM_D3DCSX: &[u8] = include_bytes!(r#"../libs\arm/d3dcsx.lib"#);
static ARM_DAVCLNT: &[u8] = include_bytes!(r#"../libs\arm/davclnt.lib"#);
static ARM_DBGENG: &[u8] = include_bytes!(r#"../libs\arm/dbgeng.lib"#);
static ARM_DBGHELP: &[u8] = include_bytes!(r#"../libs\arm/dbghelp.lib"#);
static ARM_DBGMODEL: &[u8] = include_bytes!(r#"../libs\arm/dbgmodel.lib"#);
static ARM_DCIMAN32: &[u8] = include_bytes!(r#"../libs\arm/DCIMAN32.lib"#);
static ARM_DCOMP: &[u8] = include_bytes!(r#"../libs\arm/dcomp.lib"#);
static ARM_DDRAW: &[u8] = include_bytes!(r#"../libs\arm/DDRAW.lib"#);
static ARM_DEVICEACCESS: &[u8] = include_bytes!(r#"../libs\arm/deviceaccess.lib"#);
static ARM_DFLAYOUT: &[u8] = include_bytes!(r#"../libs\arm/dflayout.lib"#);
static ARM_DHCPCSVC: &[u8] = include_bytes!(r#"../libs\arm/dhcpcsvc.lib"#);
static ARM_DHCPCSVC6: &[u8] = include_bytes!(r#"../libs\arm/dhcpcsvc6.lib"#);
static ARM_DHCPSAPI: &[u8] = include_bytes!(r#"../libs\arm/DHCPSAPI.lib"#);
static ARM_DIAGNOSTICDATAQUERY: &[u8] = include_bytes!(r#"../libs\arm/DiagnosticDataQuery.lib"#);
static ARM_DINPUT8: &[u8] = include_bytes!(r#"../libs\arm/DINPUT8.lib"#);
static ARM_DIRECTML: &[u8] = include_bytes!(r#"../libs\arm/DirectML.lib"#);
static ARM_DNSAPI: &[u8] = include_bytes!(r#"../libs\arm/DNSAPI.lib"#);
static ARM_DRT: &[u8] = include_bytes!(r#"../libs\arm/drt.lib"#);
static ARM_DRTPROV: &[u8] = include_bytes!(r#"../libs\arm/drtprov.lib"#);
static ARM_DRTTRANSPORT: &[u8] = include_bytes!(r#"../libs\arm/drttransport.lib"#);
static ARM_DSOUND: &[u8] = include_bytes!(r#"../libs\arm/DSOUND.lib"#);
static ARM_DSPARSE: &[u8] = include_bytes!(r#"../libs\arm/DSPARSE.lib"#);
static ARM_DSPROP: &[u8] = include_bytes!(r#"../libs\arm/dsprop.lib"#);
static ARM_DSSEC: &[u8] = include_bytes!(r#"../libs\arm/DSSEC.lib"#);
static ARM_DSUIEXT: &[u8] = include_bytes!(r#"../libs\arm/dsuiext.lib"#);
static ARM_DWMAPI: &[u8] = include_bytes!(r#"../libs\arm/dwmapi.lib"#);
static ARM_DWRITE: &[u8] = include_bytes!(r#"../libs\arm/DWrite.lib"#);
static ARM_DXGI: &[u8] = include_bytes!(r#"../libs\arm/dxgi.lib"#);
static ARM_DXVA2: &[u8] = include_bytes!(r#"../libs\arm/dxva2.lib"#);
static ARM_EAPPCFG: &[u8] = include_bytes!(r#"../libs\arm/eappcfg.lib"#);
static ARM_EAPPPRXY: &[u8] = include_bytes!(r#"../libs\arm/eappprxy.lib"#);
static ARM_EFSWRT: &[u8] = include_bytes!(r#"../libs\arm/efswrt.lib"#);
static ARM_ELSCORE: &[u8] = include_bytes!(r#"../libs\arm/elscore.lib"#);
static ARM_ESENT: &[u8] = include_bytes!(r#"../libs\arm/ESENT.lib"#);
static ARM_EVR: &[u8] = include_bytes!(r#"../libs\arm/EVR.lib"#);
static ARM_FAULTREP: &[u8] = include_bytes!(r#"../libs\arm/faultrep.lib"#);
static ARM_FHSVCCTL: &[u8] = include_bytes!(r#"../libs\arm/fhsvcctl.lib"#);
static ARM_FLTLIB: &[u8] = include_bytes!(r#"../libs\arm/FLTLIB.lib"#);
static ARM_FONTSUB: &[u8] = include_bytes!(r#"../libs\arm/FONTSUB.lib"#);
static ARM_FWPUCLNT: &[u8] = include_bytes!(r#"../libs\arm/fwpuclnt.lib"#);
static ARM_FXSUTILITY: &[u8] = include_bytes!(r#"../libs\arm/fxsutility.lib"#);
static ARM_GDI32: &[u8] = include_bytes!(r#"../libs\arm/GDI32.lib"#);
static ARM_GPEDIT: &[u8] = include_bytes!(r#"../libs\arm/GPEDIT.lib"#);
static ARM_HID: &[u8] = include_bytes!(r#"../libs\arm/HID.lib"#);
static ARM_HLINK: &[u8] = include_bytes!(r#"../libs\arm/hlink.lib"#);
static ARM_HRTFAPO: &[u8] = include_bytes!(r#"../libs\arm/HrtfApo.lib"#);
static ARM_HTTPAPI: &[u8] = include_bytes!(r#"../libs\arm/HTTPAPI.lib"#);
static ARM_ICM32: &[u8] = include_bytes!(r#"../libs\arm/ICM32.lib"#);
static ARM_ICMUI: &[u8] = include_bytes!(r#"../libs\arm/ICMUI.lib"#);
static ARM_ICU: &[u8] = include_bytes!(r#"../libs\arm/icu.lib"#);
static ARM_IMM32: &[u8] = include_bytes!(r#"../libs\arm/IMM32.lib"#);
static ARM_INKOBJCORE: &[u8] = include_bytes!(r#"../libs\arm/inkobjcore.lib"#);
static ARM_IPHLPAPI: &[u8] = include_bytes!(r#"../libs\arm/IPHLPAPI.lib"#);
static ARM_ISCSIDSC: &[u8] = include_bytes!(r#"../libs\arm/ISCSIDSC.lib"#);
static ARM_KERNEL32: &[u8] = include_bytes!(r#"../libs\arm/KERNEL32.lib"#);
static ARM_KEYCREDMGR: &[u8] = include_bytes!(r#"../libs\arm/KeyCredMgr.lib"#);
static ARM_KSUSER: &[u8] = include_bytes!(r#"../libs\arm/ksuser.lib"#);
static ARM_KTMW32: &[u8] = include_bytes!(r#"../libs\arm/ktmw32.lib"#);
static ARM_LOADPERF: &[u8] = include_bytes!(r#"../libs\arm/loadperf.lib"#);
static ARM_MAGNIFICATION: &[u8] = include_bytes!(r#"../libs\arm/MAGNIFICATION.lib"#);
static ARM_MAPI32: &[u8] = include_bytes!(r#"../libs\arm/MAPI32.lib"#);
static ARM_MDMREGISTRATION: &[u8] = include_bytes!(r#"../libs\arm/MDMRegistration.lib"#);
static ARM_MF: &[u8] = include_bytes!(r#"../libs\arm/MF.lib"#);
static ARM_MFCORE: &[u8] = include_bytes!(r#"../libs\arm/MFCORE.lib"#);
static ARM_MFPLAT: &[u8] = include_bytes!(r#"../libs\arm/MFPlat.lib"#);
static ARM_MFPLAY: &[u8] = include_bytes!(r#"../libs\arm/MFPlay.lib"#);
static ARM_MFREADWRITE: &[u8] = include_bytes!(r#"../libs\arm/MFReadWrite.lib"#);
static ARM_MFSENSORGROUP: &[u8] = include_bytes!(r#"../libs\arm/MFSENSORGROUP.lib"#);
static ARM_MFSRCSNK: &[u8] = include_bytes!(r#"../libs\arm/mfsrcsnk.lib"#);
static ARM_MGMTAPI: &[u8] = include_bytes!(r#"../libs\arm/mgmtapi.lib"#);
static ARM_MI: &[u8] = include_bytes!(r#"../libs\arm/mi.lib"#);
static ARM_MMDEVAPI: &[u8] = include_bytes!(r#"../libs\arm/MMDevAPI.lib"#);
static ARM_MPR: &[u8] = include_bytes!(r#"../libs\arm/MPR.lib"#);
static ARM_MPRAPI: &[u8] = include_bytes!(r#"../libs\arm/MPRAPI.lib"#);
static ARM_MRMSUPPORT: &[u8] = include_bytes!(r#"../libs\arm/MrmSupport.lib"#);
static ARM_MSACM32: &[u8] = include_bytes!(r#"../libs\arm/MSACM32.lib"#);
static ARM_MSAJAPI: &[u8] = include_bytes!(r#"../libs\arm/MSAJApi.lib"#);
static ARM_MSCMS: &[u8] = include_bytes!(r#"../libs\arm/mscms.lib"#);
static ARM_MSCTFMONITOR: &[u8] = include_bytes!(r#"../libs\arm/MsCtfMonitor.lib"#);
static ARM_MSDMO: &[u8] = include_bytes!(r#"../libs\arm/msdmo.lib"#);
static ARM_MSDRM: &[u8] = include_bytes!(r#"../libs\arm/msdrm.lib"#);
static ARM_MSI: &[u8] = include_bytes!(r#"../libs\arm/msi.lib"#);
static ARM_MSIMG32: &[u8] = include_bytes!(r#"../libs\arm/MSIMG32.lib"#);
static ARM_MSPORTS: &[u8] = include_bytes!(r#"../libs\arm/MSPORTS.lib"#);
static ARM_MSTASK: &[u8] = include_bytes!(r#"../libs\arm/mstask.lib"#);
static ARM_MSVFW32: &[u8] = include_bytes!(r#"../libs\arm/MSVFW32.lib"#);
static ARM_MSWSOCK: &[u8] = include_bytes!(r#"../libs\arm/MSWSOCK.lib"#);
static ARM_MTXDM: &[u8] = include_bytes!(r#"../libs\arm/MTxDM.lib"#);
static ARM_NCRYPT: &[u8] = include_bytes!(r#"../libs\arm/ncrypt.lib"#);
static ARM_NDFAPI: &[u8] = include_bytes!(r#"../libs\arm/NDFAPI.lib"#);
static ARM_NETAPI32: &[u8] = include_bytes!(r#"../libs\arm/NETAPI32.lib"#);
static ARM_NETSH: &[u8] = include_bytes!(r#"../libs\arm/NETSH.lib"#);
static ARM_NEWDEV: &[u8] = include_bytes!(r#"../libs\arm/newdev.lib"#);
static ARM_NINPUT: &[u8] = include_bytes!(r#"../libs\arm/NInput.lib"#);
static ARM_NORMALIZ: &[u8] = include_bytes!(r#"../libs\arm/NORMALIZ.lib"#);
static ARM_NTDLL: &[u8] = include_bytes!(r#"../libs\arm/ntdll.lib"#);
static ARM_NTDSAPI: &[u8] = include_bytes!(r#"../libs\arm/NTDSAPI.lib"#);
static ARM_NTLANMAN: &[u8] = include_bytes!(r#"../libs\arm/NTLANMAN.lib"#);
static ARM_OLE32: &[u8] = include_bytes!(r#"../libs\arm/OLE32.lib"#);
static ARM_OLEACC: &[u8] = include_bytes!(r#"../libs\arm/OLEACC.lib"#);
static ARM_OLEAUT32: &[u8] = include_bytes!(r#"../libs\arm/OLEAUT32.lib"#);
static ARM_OLEDLG: &[u8] = include_bytes!(r#"../libs\arm/oledlg.lib"#);
static ARM_ONDEMANDCONNROUTEHELPER: &[u8] = include_bytes!(r#"../libs\arm/OnDemandConnRouteHelper.lib"#);
static ARM_OPENGL32: &[u8] = include_bytes!(r#"../libs\arm/OPENGL32.lib"#);
static ARM_P2P: &[u8] = include_bytes!(r#"../libs\arm/P2P.lib"#);
static ARM_P2PGRAPH: &[u8] = include_bytes!(r#"../libs\arm/P2PGRAPH.lib"#);
static ARM_PDH: &[u8] = include_bytes!(r#"../libs\arm/pdh.lib"#);
static ARM_PEERDIST: &[u8] = include_bytes!(r#"../libs\arm/PeerDist.lib"#);
static ARM_POWRPROF: &[u8] = include_bytes!(r#"../libs\arm/POWRPROF.lib"#);
static ARM_PRNTVPT: &[u8] = include_bytes!(r#"../libs\arm/prntvpt.lib"#);
static ARM_PROJECTEDFSLIB: &[u8] = include_bytes!(r#"../libs\arm/PROJECTEDFSLIB.lib"#);
static ARM_PROPSYS: &[u8] = include_bytes!(r#"../libs\arm/PROPSYS.lib"#);
static ARM_QUARTZ: &[u8] = include_bytes!(r#"../libs\arm/QUARTZ.lib"#);
static ARM_QUERY: &[u8] = include_bytes!(r#"../libs\arm/query.lib"#);
static ARM_QWAVE: &[u8] = include_bytes!(r#"../libs\arm/qwave.lib"#);
static ARM_RASAPI32: &[u8] = include_bytes!(r#"../libs\arm/RASAPI32.lib"#);
static ARM_RASDLG: &[u8] = include_bytes!(r#"../libs\arm/RASDLG.lib"#);
static ARM_RESUTILS: &[u8] = include_bytes!(r#"../libs\arm/RESUTILS.lib"#);
static ARM_ROMETADATA: &[u8] = include_bytes!(r#"../libs\arm/RoMetadata.lib"#);
static ARM_RPCNS4: &[u8] = include_bytes!(r#"../libs\arm/RPCNS4.lib"#);
static ARM_RPCRT4: &[u8] = include_bytes!(r#"../libs\arm/RPCRT4.lib"#);
static ARM_RSTRTMGR: &[u8] = include_bytes!(r#"../libs\arm/RstrtMgr.lib"#);
static ARM_RTM: &[u8] = include_bytes!(r#"../libs\arm/rtm.lib"#);
static ARM_SAS: &[u8] = include_bytes!(r#"../libs\arm/SAS.lib"#);
static ARM_SCARDDLG: &[u8] = include_bytes!(r#"../libs\arm/SCARDDLG.lib"#);
static ARM_SCHANNEL: &[u8] = include_bytes!(r#"../libs\arm/SCHANNEL.lib"#);
static ARM_SECUR32: &[u8] = include_bytes!(r#"../libs\arm/SECUR32.lib"#);
static ARM_SENSAPI: &[u8] = include_bytes!(r#"../libs\arm/SensApi.lib"#);
static ARM_SETUPAPI: &[u8] = include_bytes!(r#"../libs\arm/SETUPAPI.lib"#);
static ARM_SFC: &[u8] = include_bytes!(r#"../libs\arm/sfc.lib"#);
static ARM_SHDOCVW: &[u8] = include_bytes!(r#"../libs\arm/SHDOCVW.lib"#);
static ARM_SHELL32: &[u8] = include_bytes!(r#"../libs\arm/SHELL32.lib"#);
static ARM_SHLWAPI: &[u8] = include_bytes!(r#"../libs\arm/SHLWAPI.lib"#);
static ARM_SLC: &[u8] = include_bytes!(r#"../libs\arm/SLC.lib"#);
static ARM_SLCEXT: &[u8] = include_bytes!(r#"../libs\arm/slcext.lib"#);
static ARM_SLWGA: &[u8] = include_bytes!(r#"../libs\arm/SLWGA.lib"#);
static ARM_SNMPAPI: &[u8] = include_bytes!(r#"../libs\arm/snmpapi.lib"#);
static ARM_SRPAPI: &[u8] = include_bytes!(r#"../libs\arm/srpapi.lib"#);
static ARM_SSPICLI: &[u8] = include_bytes!(r#"../libs\arm/SspiCli.lib"#);
static ARM_T2EMBED: &[u8] = include_bytes!(r#"../libs\arm/t2embed.lib"#);
static ARM_TAPI32: &[u8] = include_bytes!(r#"../libs\arm/TAPI32.lib"#);
static ARM_TBS: &[u8] = include_bytes!(r#"../libs\arm/tbs.lib"#);
static ARM_TDH: &[u8] = include_bytes!(r#"../libs\arm/TDH.lib"#);
static ARM_TOKENBINDING: &[u8] = include_bytes!(r#"../libs\arm/TOKENBINDING.lib"#);
static ARM_TRAFFIC: &[u8] = include_bytes!(r#"../libs\arm/TRAFFIC.lib"#);
static ARM_TXFW32: &[u8] = include_bytes!(r#"../libs\arm/txfw32.lib"#);
static ARM_UALAPI: &[u8] = include_bytes!(r#"../libs\arm/ualapi.lib"#);
static ARM_UIAUTOMATIONCORE: &[u8] = include_bytes!(r#"../libs\arm/UIAutomationCore.lib"#);
static ARM_URLMON: &[u8] = include_bytes!(r#"../libs\arm/URLMON.lib"#);
static ARM_USER32: &[u8] = include_bytes!(r#"../libs\arm/USER32.lib"#);
static ARM_USERENV: &[u8] = include_bytes!(r#"../libs\arm/USERENV.lib"#);
static ARM_USP10: &[u8] = include_bytes!(r#"../libs\arm/USP10.lib"#);
static ARM_UXTHEME: &[u8] = include_bytes!(r#"../libs\arm/UXTHEME.lib"#);
static ARM_VERIFIER: &[u8] = include_bytes!(r#"../libs\arm/verifier.lib"#);
static ARM_VERSION: &[u8] = include_bytes!(r#"../libs\arm/VERSION.lib"#);
static ARM_VERTDLL: &[u8] = include_bytes!(r#"../libs\arm/vertdll.lib"#);
static ARM_VIRTDISK: &[u8] = include_bytes!(r#"../libs\arm/VirtDisk.lib"#);
static ARM_VSSAPI: &[u8] = include_bytes!(r#"../libs\arm/VSSAPI.lib"#);
static ARM_WCMAPI: &[u8] = include_bytes!(r#"../libs\arm/wcmapi.lib"#);
static ARM_WDSBP: &[u8] = include_bytes!(r#"../libs\arm/WDSBP.lib"#);
static ARM_WDSCLIENTAPI: &[u8] = include_bytes!(r#"../libs\arm/WDSCLIENTAPI.lib"#);
static ARM_WDSMC: &[u8] = include_bytes!(r#"../libs\arm/WDSMC.lib"#);
static ARM_WDSPXE: &[u8] = include_bytes!(r#"../libs\arm/WDSPXE.lib"#);
static ARM_WDSTPTC: &[u8] = include_bytes!(r#"../libs\arm/WDSTPTC.lib"#);
static ARM_WEBAUTHN: &[u8] = include_bytes!(r#"../libs\arm/webauthn.lib"#);
static ARM_WEBSERVICES: &[u8] = include_bytes!(r#"../libs\arm/webservices.lib"#);
static ARM_WEBSOCKET: &[u8] = include_bytes!(r#"../libs\arm/websocket.lib"#);
static ARM_WECAPI: &[u8] = include_bytes!(r#"../libs\arm/WecApi.lib"#);
static ARM_WER: &[u8] = include_bytes!(r#"../libs\arm/wer.lib"#);
static ARM_WEVTAPI: &[u8] = include_bytes!(r#"../libs\arm/wevtapi.lib"#);
static ARM_WINBIO: &[u8] = include_bytes!(r#"../libs\arm/winbio.lib"#);
static ARM_WINDOWS_AI_MACHINELEARNING: &[u8] = include_bytes!(r#"../libs\arm/windows.ai.machinelearning.lib"#);
static ARM_WINDOWS_DATA_PDF: &[u8] = include_bytes!(r#"../libs\arm/Windows.Data.Pdf.lib"#);
static ARM_WINDOWS_NETWORKING: &[u8] = include_bytes!(r#"../libs\arm/Windows.Networking.lib"#);
static ARM_WINDOWS_UI_XAML: &[u8] = include_bytes!(r#"../libs\arm/Windows.UI.Xaml.lib"#);
static ARM_WINDOWSCODECS: &[u8] = include_bytes!(r#"../libs\arm/WindowsCodecs.lib"#);
static ARM_WINFAX: &[u8] = include_bytes!(r#"../libs\arm/WINFAX.lib"#);
static ARM_WINHTTP: &[u8] = include_bytes!(r#"../libs\arm/WINHTTP.lib"#);
static ARM_WINHVEMULATION: &[u8] = include_bytes!(r#"../libs\arm/WinHvEmulation.lib"#);
static ARM_WINHVPLATFORM: &[u8] = include_bytes!(r#"../libs\arm/WinHvPlatform.lib"#);
static ARM_WININET: &[u8] = include_bytes!(r#"../libs\arm/WININET.lib"#);
static ARM_WINML: &[u8] = include_bytes!(r#"../libs\arm/winml.lib"#);
static ARM_WINMM: &[u8] = include_bytes!(r#"../libs\arm/WINMM.lib"#);
static ARM_WINSCARD: &[u8] = include_bytes!(r#"../libs\arm/WinSCard.lib"#);
static ARM_WINSPOOL: &[u8] = include_bytes!(r#"../libs\arm/WINSPOOL.lib"#);
static ARM_WINTRUST: &[u8] = include_bytes!(r#"../libs\arm/WINTRUST.lib"#);
static ARM_WINUSB: &[u8] = include_bytes!(r#"../libs\arm/WINUSB.lib"#);
static ARM_WLANAPI: &[u8] = include_bytes!(r#"../libs\arm/wlanapi.lib"#);
static ARM_WLANUI: &[u8] = include_bytes!(r#"../libs\arm/wlanui.lib"#);
static ARM_WLDAP32: &[u8] = include_bytes!(r#"../libs\arm/WLDAP32.lib"#);
static ARM_WLDP: &[u8] = include_bytes!(r#"../libs\arm/Wldp.lib"#);
static ARM_WMVCORE: &[u8] = include_bytes!(r#"../libs\arm/WMVCore.lib"#);
static ARM_WNVAPI: &[u8] = include_bytes!(r#"../libs\arm/wnvapi.lib"#);
static ARM_WOFUTIL: &[u8] = include_bytes!(r#"../libs\arm/WOFUTIL.lib"#);
static ARM_WS2_32: &[u8] = include_bytes!(r#"../libs\arm/WS2_32.lib"#);
static ARM_WSCAPI: &[u8] = include_bytes!(r#"../libs\arm/WSCAPI.lib"#);
static ARM_WSCLIENT: &[u8] = include_bytes!(r#"../libs\arm/WSClient.lib"#);
static ARM_WSDAPI: &[u8] = include_bytes!(r#"../libs\arm/wsdapi.lib"#);
static ARM_WSMSVC: &[u8] = include_bytes!(r#"../libs\arm/WsmSvc.lib"#);
static ARM_WSNMP32: &[u8] = include_bytes!(r#"../libs\arm/wsnmp32.lib"#);
static ARM_WTSAPI32: &[u8] = include_bytes!(r#"../libs\arm/WTSAPI32.lib"#);
static ARM_XAUDIO2_8: &[u8] = include_bytes!(r#"../libs\arm/XAudio2_8.lib"#);
static ARM_XINPUTUAP: &[u8] = include_bytes!(r#"../libs\arm/XINPUTUAP.lib"#);
static ARM_XMLLITE: &[u8] = include_bytes!(r#"../libs\arm/XmlLite.lib"#);
static ARM64_ACLUI: &[u8] = include_bytes!(r#"../libs\arm64/ACLUI.lib"#);
static ARM64_ACTIVEDS: &[u8] = include_bytes!(r#"../libs\arm64/ACTIVEDS.lib"#);
static ARM64_ADVAPI32: &[u8] = include_bytes!(r#"../libs\arm64/ADVAPI32.lib"#);
static ARM64_ADVPACK: &[u8] = include_bytes!(r#"../libs\arm64/ADVPACK.lib"#);
static ARM64_AMSI: &[u8] = include_bytes!(r#"../libs\arm64/Amsi.lib"#);
static ARM64_API_MS_WIN_APPMODEL_RUNTIME_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-appmodel-runtime-l1-1-1.lib"#);
static ARM64_API_MS_WIN_APPMODEL_RUNTIME_L1_1_3: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-appmodel-runtime-l1-1-3.lib"#);
static ARM64_API_MS_WIN_CORE_APIQUERY_L2_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-apiquery-l2-1-0.lib"#);
static ARM64_API_MS_WIN_CORE_BACKGROUNDTASK_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-backgroundtask-l1-1-0.lib"#);
static ARM64_API_MS_WIN_CORE_COMM_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-comm-l1-1-1.lib"#);
static ARM64_API_MS_WIN_CORE_COMM_L1_1_2: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-comm-l1-1-2.lib"#);
static ARM64_API_MS_WIN_CORE_ERRORHANDLING_L1_1_3: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-errorhandling-l1-1-3.lib"#);
static ARM64_API_MS_WIN_CORE_FEATURESTAGING_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-featurestaging-l1-1-0.lib"#);
static ARM64_API_MS_WIN_CORE_FEATURESTAGING_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-featurestaging-l1-1-1.lib"#);
static ARM64_API_MS_WIN_CORE_FILE_FROMAPP_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-file-fromapp-l1-1-0.lib"#);
static ARM64_API_MS_WIN_CORE_HANDLE_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-handle-l1-1-0.lib"#);
static ARM64_API_MS_WIN_CORE_LIBRARYLOADER_L2_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-libraryloader-l2-1-0.lib"#);
static ARM64_API_MS_WIN_CORE_MEMORY_L1_1_3: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-memory-l1-1-3.lib"#);
static ARM64_API_MS_WIN_CORE_MEMORY_L1_1_4: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-memory-l1-1-4.lib"#);
static ARM64_API_MS_WIN_CORE_MEMORY_L1_1_5: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-memory-l1-1-5.lib"#);
static ARM64_API_MS_WIN_CORE_MEMORY_L1_1_6: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-memory-l1-1-6.lib"#);
static ARM64_API_MS_WIN_CORE_MEMORY_L1_1_7: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-memory-l1-1-7.lib"#);
static ARM64_API_MS_WIN_CORE_PATH_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-path-l1-1-0.lib"#);
static ARM64_API_MS_WIN_CORE_PSM_APPNOTIFY_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-psm-appnotify-l1-1-0.lib"#);
static ARM64_API_MS_WIN_CORE_PSM_APPNOTIFY_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-psm-appnotify-l1-1-1.lib"#);
static ARM64_API_MS_WIN_CORE_REALTIME_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-realtime-l1-1-1.lib"#);
static ARM64_API_MS_WIN_CORE_REALTIME_L1_1_2: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-realtime-l1-1-2.lib"#);
static ARM64_API_MS_WIN_CORE_SLAPI_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-slapi-l1-1-0.lib"#);
static ARM64_API_MS_WIN_CORE_STATE_HELPERS_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-state-helpers-l1-1-0.lib"#);
static ARM64_API_MS_WIN_CORE_SYSINFO_L1_2_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-sysinfo-l1-2-0.lib"#);
static ARM64_API_MS_WIN_CORE_SYSINFO_L1_2_3: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-sysinfo-l1-2-3.lib"#);
static ARM64_API_MS_WIN_CORE_SYSINFO_L1_2_4: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-sysinfo-l1-2-4.lib"#);
static ARM64_API_MS_WIN_CORE_UTIL_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-util-l1-1-1.lib"#);
static ARM64_API_MS_WIN_CORE_WINRT_ERROR_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-winrt-error-l1-1-0.lib"#);
static ARM64_API_MS_WIN_CORE_WINRT_ERROR_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-winrt-error-l1-1-1.lib"#);
static ARM64_API_MS_WIN_CORE_WINRT_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-winrt-l1-1-0.lib"#);
static ARM64_API_MS_WIN_CORE_WINRT_REGISTRATION_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-winrt-registration-l1-1-0.lib"#);
static ARM64_API_MS_WIN_CORE_WINRT_ROBUFFER_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-winrt-robuffer-l1-1-0.lib"#);
static ARM64_API_MS_WIN_CORE_WINRT_ROPARAMETERIZEDIID_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-winrt-roparameterizediid-l1-1-0.lib"#);
static ARM64_API_MS_WIN_CORE_WINRT_STRING_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-winrt-string-l1-1-0.lib"#);
static ARM64_API_MS_WIN_CORE_WINRT_STRING_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-winrt-string-l1-1-1.lib"#);
static ARM64_API_MS_WIN_CORE_WOW64_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-core-wow64-l1-1-1.lib"#);
static ARM64_API_MS_WIN_DEVICES_QUERY_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-devices-query-l1-1-0.lib"#);
static ARM64_API_MS_WIN_DEVICES_QUERY_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-devices-query-l1-1-1.lib"#);
static ARM64_API_MS_WIN_DX_D3DKMT_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-dx-d3dkmt-l1-1-0.lib"#);
static ARM64_API_MS_WIN_GAMING_DEVICEINFORMATION_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-gaming-deviceinformation-l1-1-0.lib"#);
static ARM64_API_MS_WIN_GAMING_EXPANDEDRESOURCES_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-gaming-expandedresources-l1-1-0.lib"#);
static ARM64_API_MS_WIN_GAMING_TCUI_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-gaming-tcui-l1-1-0.lib"#);
static ARM64_API_MS_WIN_GAMING_TCUI_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-gaming-tcui-l1-1-1.lib"#);
static ARM64_API_MS_WIN_GAMING_TCUI_L1_1_2: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-gaming-tcui-l1-1-2.lib"#);
static ARM64_API_MS_WIN_GAMING_TCUI_L1_1_3: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-gaming-tcui-l1-1-3.lib"#);
static ARM64_API_MS_WIN_GAMING_TCUI_L1_1_4: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-gaming-tcui-l1-1-4.lib"#);
static ARM64_API_MS_WIN_MM_MISC_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-mm-misc-l1-1-1.lib"#);
static ARM64_API_MS_WIN_NET_ISOLATION_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-net-isolation-l1-1-0.lib"#);
static ARM64_API_MS_WIN_SECURITY_BASE_L1_2_2: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-security-base-l1-2-2.lib"#);
static ARM64_API_MS_WIN_SECURITY_ISOLATEDCONTAINER_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-security-isolatedcontainer-l1-1-0.lib"#);
static ARM64_API_MS_WIN_SECURITY_ISOLATEDCONTAINER_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-security-isolatedcontainer-l1-1-1.lib"#);
static ARM64_API_MS_WIN_SERVICE_CORE_L1_1_3: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-service-core-l1-1-3.lib"#);
static ARM64_API_MS_WIN_SERVICE_CORE_L1_1_4: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-service-core-l1-1-4.lib"#);
static ARM64_API_MS_WIN_SHCORE_SCALING_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-shcore-scaling-l1-1-0.lib"#);
static ARM64_API_MS_WIN_SHCORE_SCALING_L1_1_1: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-shcore-scaling-l1-1-1.lib"#);
static ARM64_API_MS_WIN_SHCORE_SCALING_L1_1_2: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-shcore-scaling-l1-1-2.lib"#);
static ARM64_API_MS_WIN_SHCORE_STREAM_WINRT_L1_1_0: &[u8] = include_bytes!(r#"../libs\arm64/api-ms-win-shcore-stream-winrt-l1-1-0.lib"#);
static ARM64_APPHELP: &[u8] = include_bytes!(r#"../libs\arm64/APPHELP.lib"#);
static ARM64_AUTHZ: &[u8] = include_bytes!(r#"../libs\arm64/AUTHZ.lib"#);
static ARM64_AVICAP32: &[u8] = include_bytes!(r#"../libs\arm64/AVICAP32.lib"#);
static ARM64_AVIFIL32: &[u8] = include_bytes!(r#"../libs\arm64/AVIFIL32.lib"#);
static ARM64_BCRYPT: &[u8] = include_bytes!(r#"../libs\arm64/bcrypt.lib"#);
static ARM64_BLUETOOTHAPIS: &[u8] = include_bytes!(r#"../libs\arm64/BluetoothApis.lib"#);
static ARM64_BTHPROPS: &[u8] = include_bytes!(r#"../libs\arm64/bthprops.lib"#);
static ARM64_CABINET: &[u8] = include_bytes!(r#"../libs\arm64/Cabinet.lib"#);
static ARM64_CERTADM: &[u8] = include_bytes!(r#"../libs\arm64/certadm.lib"#);
static ARM64_CERTPOLENG: &[u8] = include_bytes!(r#"../libs\arm64/certpoleng.lib"#);
static ARM64_CFGMGR32: &[u8] = include_bytes!(r#"../libs\arm64/CFGMGR32.lib"#);
static ARM64_CHAKRA: &[u8] = include_bytes!(r#"../libs\arm64/chakra.lib"#);
static ARM64_CLDAPI: &[u8] = include_bytes!(r#"../libs\arm64/cldapi.lib"#);
static ARM64_CLFSW32: &[u8] = include_bytes!(r#"../libs\arm64/clfsw32.lib"#);
static ARM64_CLUSAPI: &[u8] = include_bytes!(r#"../libs\arm64/CLUSAPI.lib"#);
static ARM64_COMCTL32: &[u8] = include_bytes!(r#"../libs\arm64/COMCTL32.lib"#);
static ARM64_COMDLG32: &[u8] = include_bytes!(r#"../libs\arm64/COMDLG32.lib"#);
static ARM64_COMSVCS: &[u8] = include_bytes!(r#"../libs\arm64/comsvcs.lib"#);
static ARM64_COREMESSAGING: &[u8] = include_bytes!(r#"../libs\arm64/CoreMessaging.lib"#);
static ARM64_CREDUI: &[u8] = include_bytes!(r#"../libs\arm64/credui.lib"#);
static ARM64_CRYPT32: &[u8] = include_bytes!(r#"../libs\arm64/CRYPT32.lib"#);
static ARM64_CRYPTNET: &[u8] = include_bytes!(r#"../libs\arm64/CRYPTNET.lib"#);
static ARM64_CRYPTUI: &[u8] = include_bytes!(r#"../libs\arm64/CRYPTUI.lib"#);
static ARM64_CRYPTXML: &[u8] = include_bytes!(r#"../libs\arm64/CRYPTXML.lib"#);
static ARM64_CSCAPI: &[u8] = include_bytes!(r#"../libs\arm64/CSCAPI.lib"#);
static ARM64_D2D1: &[u8] = include_bytes!(r#"../libs\arm64/d2d1.lib"#);
static ARM64_D3D10: &[u8] = include_bytes!(r#"../libs\arm64/d3d10.lib"#);
static ARM64_D3D10_1: &[u8] = include_bytes!(r#"../libs\arm64/d3d10_1.lib"#);
static ARM64_D3D11: &[u8] = include_bytes!(r#"../libs\arm64/d3d11.lib"#);
static ARM64_D3D12: &[u8] = include_bytes!(r#"../libs\arm64/d3d12.lib"#);
static ARM64_D3D9: &[u8] = include_bytes!(r#"../libs\arm64/d3d9.lib"#);
static ARM64_D3DCOMPILER_47: &[u8] = include_bytes!(r#"../libs\arm64/D3DCOMPILER_47.lib"#);
static ARM64_D3DCSX: &[u8] = include_bytes!(r#"../libs\arm64/d3dcsx.lib"#);
static ARM64_DAVCLNT: &[u8] = include_bytes!(r#"../libs\arm64/davclnt.lib"#);
static ARM64_DBGENG: &[u8] = include_bytes!(r#"../libs\arm64/dbgeng.lib"#);
static ARM64_DBGHELP: &[u8] = include_bytes!(r#"../libs\arm64/dbghelp.lib"#);
static ARM64_DBGMODEL: &[u8] = include_bytes!(r#"../libs\arm64/dbgmodel.lib"#);
static ARM64_DCIMAN32: &[u8] = include_bytes!(r#"../libs\arm64/DCIMAN32.lib"#);
static ARM64_DCOMP: &[u8] = include_bytes!(r#"../libs\arm64/dcomp.lib"#);
static ARM64_DDRAW: &[u8] = include_bytes!(r#"../libs\arm64/DDRAW.lib"#);
static ARM64_DEVICEACCESS: &[u8] = include_bytes!(r#"../libs\arm64/deviceaccess.lib"#);
static ARM64_DFLAYOUT: &[u8] = include_bytes!(r#"../libs\arm64/dflayout.lib"#);
static ARM64_DHCPCSVC: &[u8] = include_bytes!(r#"../libs\arm64/dhcpcsvc.lib"#);
static ARM64_DHCPCSVC6: &[u8] = include_bytes!(r#"../libs\arm64/dhcpcsvc6.lib"#);
static ARM64_DHCPSAPI: &[u8] = include_bytes!(r#"../libs\arm64/DHCPSAPI.lib"#);
static ARM64_DIAGNOSTICDATAQUERY: &[u8] = include_bytes!(r#"../libs\arm64/DiagnosticDataQuery.lib"#);
static ARM64_DINPUT8: &[u8] = include_bytes!(r#"../libs\arm64/DINPUT8.lib"#);
static ARM64_DIRECTML: &[u8] = include_bytes!(r#"../libs\arm64/DirectML.lib"#);
static ARM64_DNSAPI: &[u8] = include_bytes!(r#"../libs\arm64/DNSAPI.lib"#);
static ARM64_DRT: &[u8] = include_bytes!(r#"../libs\arm64/drt.lib"#);
static ARM64_DRTPROV: &[u8] = include_bytes!(r#"../libs\arm64/drtprov.lib"#);
static ARM64_DRTTRANSPORT: &[u8] = include_bytes!(r#"../libs\arm64/drttransport.lib"#);
static ARM64_DSOUND: &[u8] = include_bytes!(r#"../libs\arm64/DSOUND.lib"#);
static ARM64_DSPARSE: &[u8] = include_bytes!(r#"../libs\arm64/DSPARSE.lib"#);
static ARM64_DSPROP: &[u8] = include_bytes!(r#"../libs\arm64/dsprop.lib"#);
static ARM64_DSSEC: &[u8] = include_bytes!(r#"../libs\arm64/DSSEC.lib"#);
static ARM64_DSUIEXT: &[u8] = include_bytes!(r#"../libs\arm64/dsuiext.lib"#);
static ARM64_DWMAPI: &[u8] = include_bytes!(r#"../libs\arm64/dwmapi.lib"#);
static ARM64_DWRITE: &[u8] = include_bytes!(r#"../libs\arm64/DWrite.lib"#);
static ARM64_DXGI: &[u8] = include_bytes!(r#"../libs\arm64/dxgi.lib"#);
static ARM64_DXVA2: &[u8] = include_bytes!(r#"../libs\arm64/dxva2.lib"#);
static ARM64_EAPPCFG: &[u8] = include_bytes!(r#"../libs\arm64/eappcfg.lib"#);
static ARM64_EAPPPRXY: &[u8] = include_bytes!(r#"../libs\arm64/eappprxy.lib"#);
static ARM64_EFSWRT: &[u8] = include_bytes!(r#"../libs\arm64/efswrt.lib"#);
static ARM64_ELSCORE: &[u8] = include_bytes!(r#"../libs\arm64/elscore.lib"#);
static ARM64_ESENT: &[u8] = include_bytes!(r#"../libs\arm64/ESENT.lib"#);
static ARM64_EVR: &[u8] = include_bytes!(r#"../libs\arm64/EVR.lib"#);
static ARM64_FAULTREP: &[u8] = include_bytes!(r#"../libs\arm64/faultrep.lib"#);
static ARM64_FHSVCCTL: &[u8] = include_bytes!(r#"../libs\arm64/fhsvcctl.lib"#);
static ARM64_FLTLIB: &[u8] = include_bytes!(r#"../libs\arm64/FLTLIB.lib"#);
static ARM64_FONTSUB: &[u8] = include_bytes!(r#"../libs\arm64/FONTSUB.lib"#);
static ARM64_FWPUCLNT: &[u8] = include_bytes!(r#"../libs\arm64/fwpuclnt.lib"#);
static ARM64_FXSUTILITY: &[u8] = include_bytes!(r#"../libs\arm64/fxsutility.lib"#);
static ARM64_GDI32: &[u8] = include_bytes!(r#"../libs\arm64/GDI32.lib"#);
static ARM64_GPEDIT: &[u8] = include_bytes!(r#"../libs\arm64/GPEDIT.lib"#);
static ARM64_HID: &[u8] = include_bytes!(r#"../libs\arm64/HID.lib"#);
static ARM64_HLINK: &[u8] = include_bytes!(r#"../libs\arm64/hlink.lib"#);
static ARM64_HRTFAPO: &[u8] = include_bytes!(r#"../libs\arm64/HrtfApo.lib"#);
static ARM64_HTTPAPI: &[u8] = include_bytes!(r#"../libs\arm64/HTTPAPI.lib"#);
static ARM64_ICM32: &[u8] = include_bytes!(r#"../libs\arm64/ICM32.lib"#);
static ARM64_ICMUI: &[u8] = include_bytes!(r#"../libs\arm64/ICMUI.lib"#);
static ARM64_ICU: &[u8] = include_bytes!(r#"../libs\arm64/icu.lib"#);
static ARM64_IMM32: &[u8] = include_bytes!(r#"../libs\arm64/IMM32.lib"#);
static ARM64_INKOBJCORE: &[u8] = include_bytes!(r#"../libs\arm64/inkobjcore.lib"#);
static ARM64_IPHLPAPI: &[u8] = include_bytes!(r#"../libs\arm64/IPHLPAPI.lib"#);
static ARM64_ISCSIDSC: &[u8] = include_bytes!(r#"../libs\arm64/ISCSIDSC.lib"#);
static ARM64_KERNEL32: &[u8] = include_bytes!(r#"../libs\arm64/KERNEL32.lib"#);
static ARM64_KEYCREDMGR: &[u8] = include_bytes!(r#"../libs\arm64/KeyCredMgr.lib"#);
static ARM64_KSUSER: &[u8] = include_bytes!(r#"../libs\arm64/ksuser.lib"#);
static ARM64_KTMW32: &[u8] = include_bytes!(r#"../libs\arm64/ktmw32.lib"#);
static ARM64_LOADPERF: &[u8] = include_bytes!(r#"../libs\arm64/loadperf.lib"#);
static ARM64_MAGNIFICATION: &[u8] = include_bytes!(r#"../libs\arm64/MAGNIFICATION.lib"#);
static ARM64_MAPI32: &[u8] = include_bytes!(r#"../libs\arm64/MAPI32.lib"#);
static ARM64_MDMREGISTRATION: &[u8] = include_bytes!(r#"../libs\arm64/MDMRegistration.lib"#);
static ARM64_MF: &[u8] = include_bytes!(r#"../libs\arm64/MF.lib"#);
static ARM64_MFCORE: &[u8] = include_bytes!(r#"../libs\arm64/MFCORE.lib"#);
static ARM64_MFPLAT: &[u8] = include_bytes!(r#"../libs\arm64/MFPlat.lib"#);
static ARM64_MFPLAY: &[u8] = include_bytes!(r#"../libs\arm64/MFPlay.lib"#);
static ARM64_MFREADWRITE: &[u8] = include_bytes!(r#"../libs\arm64/MFReadWrite.lib"#);
static ARM64_MFSENSORGROUP: &[u8] = include_bytes!(r#"../libs\arm64/MFSENSORGROUP.lib"#);
static ARM64_MFSRCSNK: &[u8] = include_bytes!(r#"../libs\arm64/mfsrcsnk.lib"#);
static ARM64_MGMTAPI: &[u8] = include_bytes!(r#"../libs\arm64/mgmtapi.lib"#);
static ARM64_MI: &[u8] = include_bytes!(r#"../libs\arm64/mi.lib"#);
static ARM64_MMDEVAPI: &[u8] = include_bytes!(r#"../libs\arm64/MMDevAPI.lib"#);
static ARM64_MPR: &[u8] = include_bytes!(r#"../libs\arm64/MPR.lib"#);
static ARM64_MPRAPI: &[u8] = include_bytes!(r#"../libs\arm64/MPRAPI.lib"#);
static ARM64_MRMSUPPORT: &[u8] = include_bytes!(r#"../libs\arm64/MrmSupport.lib"#);
static ARM64_MSACM32: &[u8] = include_bytes!(r#"../libs\arm64/MSACM32.lib"#);
static ARM64_MSAJAPI: &[u8] = include_bytes!(r#"../libs\arm64/MSAJApi.lib"#);
static ARM64_MSCMS: &[u8] = include_bytes!(r#"../libs\arm64/mscms.lib"#);
static ARM64_MSCTFMONITOR: &[u8] = include_bytes!(r#"../libs\arm64/MsCtfMonitor.lib"#);
static ARM64_MSDMO: &[u8] = include_bytes!(r#"../libs\arm64/msdmo.lib"#);
static ARM64_MSDRM: &[u8] = include_bytes!(r#"../libs\arm64/msdrm.lib"#);
static ARM64_MSI: &[u8] = include_bytes!(r#"../libs\arm64/msi.lib"#);
static ARM64_MSIMG32: &[u8] = include_bytes!(r#"../libs\arm64/MSIMG32.lib"#);
static ARM64_MSPORTS: &[u8] = include_bytes!(r#"../libs\arm64/MSPORTS.lib"#);
static ARM64_MSTASK: &[u8] = include_bytes!(r#"../libs\arm64/mstask.lib"#);
static ARM64_MSVFW32: &[u8] = include_bytes!(r#"../libs\arm64/MSVFW32.lib"#);
static ARM64_MSWSOCK: &[u8] = include_bytes!(r#"../libs\arm64/MSWSOCK.lib"#);
static ARM64_MTXDM: &[u8] = include_bytes!(r#"../libs\arm64/MTxDM.lib"#);
static ARM64_NCRYPT: &[u8] = include_bytes!(r#"../libs\arm64/ncrypt.lib"#);
static ARM64_NDFAPI: &[u8] = include_bytes!(r#"../libs\arm64/NDFAPI.lib"#);
static ARM64_NETAPI32: &[u8] = include_bytes!(r#"../libs\arm64/NETAPI32.lib"#);
static ARM64_NETSH: &[u8] = include_bytes!(r#"../libs\arm64/NETSH.lib"#);
static ARM64_NEWDEV: &[u8] = include_bytes!(r#"../libs\arm64/newdev.lib"#);
static ARM64_NINPUT: &[u8] = include_bytes!(r#"../libs\arm64/NInput.lib"#);
static ARM64_NORMALIZ: &[u8] = include_bytes!(r#"../libs\arm64/NORMALIZ.lib"#);
static ARM64_NTDLL: &[u8] = include_bytes!(r#"../libs\arm64/ntdll.lib"#);
static ARM64_NTDSAPI: &[u8] = include_bytes!(r#"../libs\arm64/NTDSAPI.lib"#);
static ARM64_NTLANMAN: &[u8] = include_bytes!(r#"../libs\arm64/NTLANMAN.lib"#);
static ARM64_OLE32: &[u8] = include_bytes!(r#"../libs\arm64/OLE32.lib"#);
static ARM64_OLEACC: &[u8] = include_bytes!(r#"../libs\arm64/OLEACC.lib"#);
static ARM64_OLEAUT32: &[u8] = include_bytes!(r#"../libs\arm64/OLEAUT32.lib"#);
static ARM64_OLEDLG: &[u8] = include_bytes!(r#"../libs\arm64/oledlg.lib"#);
static ARM64_ONDEMANDCONNROUTEHELPER: &[u8] = include_bytes!(r#"../libs\arm64/OnDemandConnRouteHelper.lib"#);
static ARM64_OPENGL32: &[u8] = include_bytes!(r#"../libs\arm64/OPENGL32.lib"#);
static ARM64_P2P: &[u8] = include_bytes!(r#"../libs\arm64/P2P.lib"#);
static ARM64_P2PGRAPH: &[u8] = include_bytes!(r#"../libs\arm64/P2PGRAPH.lib"#);
static ARM64_PDH: &[u8] = include_bytes!(r#"../libs\arm64/pdh.lib"#);
static ARM64_PEERDIST: &[u8] = include_bytes!(r#"../libs\arm64/PeerDist.lib"#);
static ARM64_POWRPROF: &[u8] = include_bytes!(r#"../libs\arm64/POWRPROF.lib"#);
static ARM64_PRNTVPT: &[u8] = include_bytes!(r#"../libs\arm64/prntvpt.lib"#);
static ARM64_PROJECTEDFSLIB: &[u8] = include_bytes!(r#"../libs\arm64/PROJECTEDFSLIB.lib"#);
static ARM64_PROPSYS: &[u8] = include_bytes!(r#"../libs\arm64/PROPSYS.lib"#);
static ARM64_QUARTZ: &[u8] = include_bytes!(r#"../libs\arm64/QUARTZ.lib"#);
static ARM64_QUERY: &[u8] = include_bytes!(r#"../libs\arm64/query.lib"#);
static ARM64_QWAVE: &[u8] = include_bytes!(r#"../libs\arm64/qwave.lib"#);
static ARM64_RASAPI32: &[u8] = include_bytes!(r#"../libs\arm64/RASAPI32.lib"#);
static ARM64_RASDLG: &[u8] = include_bytes!(r#"../libs\arm64/RASDLG.lib"#);
static ARM64_RESUTILS: &[u8] = include_bytes!(r#"../libs\arm64/RESUTILS.lib"#);
static ARM64_ROMETADATA: &[u8] = include_bytes!(r#"../libs\arm64/RoMetadata.lib"#);
static ARM64_RPCNS4: &[u8] = include_bytes!(r#"../libs\arm64/RPCNS4.lib"#);
static ARM64_RPCRT4: &[u8] = include_bytes!(r#"../libs\arm64/RPCRT4.lib"#);
static ARM64_RSTRTMGR: &[u8] = include_bytes!(r#"../libs\arm64/RstrtMgr.lib"#);
static ARM64_RTM: &[u8] = include_bytes!(r#"../libs\arm64/rtm.lib"#);
static ARM64_SAS: &[u8] = include_bytes!(r#"../libs\arm64/SAS.lib"#);
static ARM64_SCARDDLG: &[u8] = include_bytes!(r#"../libs\arm64/SCARDDLG.lib"#);
static ARM64_SCHANNEL: &[u8] = include_bytes!(r#"../libs\arm64/SCHANNEL.lib"#);
static ARM64_SECUR32: &[u8] = include_bytes!(r#"../libs\arm64/SECUR32.lib"#);
static ARM64_SENSAPI: &[u8] = include_bytes!(r#"../libs\arm64/SensApi.lib"#);
static ARM64_SETUPAPI: &[u8] = include_bytes!(r#"../libs\arm64/SETUPAPI.lib"#);
static ARM64_SFC: &[u8] = include_bytes!(r#"../libs\arm64/sfc.lib"#);
static ARM64_SHDOCVW: &[u8] = include_bytes!(r#"../libs\arm64/SHDOCVW.lib"#);
static ARM64_SHELL32: &[u8] = include_bytes!(r#"../libs\arm64/SHELL32.lib"#);
static ARM64_SHLWAPI: &[u8] = include_bytes!(r#"../libs\arm64/SHLWAPI.lib"#);
static ARM64_SLC: &[u8] = include_bytes!(r#"../libs\arm64/SLC.lib"#);
static ARM64_SLCEXT: &[u8] = include_bytes!(r#"../libs\arm64/slcext.lib"#);
static ARM64_SLWGA: &[u8] = include_bytes!(r#"../libs\arm64/SLWGA.lib"#);
static ARM64_SNMPAPI: &[u8] = include_bytes!(r#"../libs\arm64/snmpapi.lib"#);
static ARM64_SRPAPI: &[u8] = include_bytes!(r#"../libs\arm64/srpapi.lib"#);
static ARM64_SSPICLI: &[u8] = include_bytes!(r#"../libs\arm64/SspiCli.lib"#);
static ARM64_T2EMBED: &[u8] = include_bytes!(r#"../libs\arm64/t2embed.lib"#);
static ARM64_TAPI32: &[u8] = include_bytes!(r#"../libs\arm64/TAPI32.lib"#);
static ARM64_TBS: &[u8] = include_bytes!(r#"../libs\arm64/tbs.lib"#);
static ARM64_TDH: &[u8] = include_bytes!(r#"../libs\arm64/TDH.lib"#);
static ARM64_TOKENBINDING: &[u8] = include_bytes!(r#"../libs\arm64/TOKENBINDING.lib"#);
static ARM64_TRAFFIC: &[u8] = include_bytes!(r#"../libs\arm64/TRAFFIC.lib"#);
static ARM64_TXFW32: &[u8] = include_bytes!(r#"../libs\arm64/txfw32.lib"#);
static ARM64_UALAPI: &[u8] = include_bytes!(r#"../libs\arm64/ualapi.lib"#);
static ARM64_UIAUTOMATIONCORE: &[u8] = include_bytes!(r#"../libs\arm64/UIAutomationCore.lib"#);
static ARM64_URLMON: &[u8] = include_bytes!(r#"../libs\arm64/URLMON.lib"#);
static ARM64_USER32: &[u8] = include_bytes!(r#"../libs\arm64/USER32.lib"#);
static ARM64_USERENV: &[u8] = include_bytes!(r#"../libs\arm64/USERENV.lib"#);
static ARM64_USP10: &[u8] = include_bytes!(r#"../libs\arm64/USP10.lib"#);
static ARM64_UXTHEME: &[u8] = include_bytes!(r#"../libs\arm64/UXTHEME.lib"#);
static ARM64_VERIFIER: &[u8] = include_bytes!(r#"../libs\arm64/verifier.lib"#);
static ARM64_VERSION: &[u8] = include_bytes!(r#"../libs\arm64/VERSION.lib"#);
static ARM64_VERTDLL: &[u8] = include_bytes!(r#"../libs\arm64/vertdll.lib"#);
static ARM64_VIRTDISK: &[u8] = include_bytes!(r#"../libs\arm64/VirtDisk.lib"#);
static ARM64_VSSAPI: &[u8] = include_bytes!(r#"../libs\arm64/VSSAPI.lib"#);
static ARM64_WCMAPI: &[u8] = include_bytes!(r#"../libs\arm64/wcmapi.lib"#);
static ARM64_WDSBP: &[u8] = include_bytes!(r#"../libs\arm64/WDSBP.lib"#);
static ARM64_WDSCLIENTAPI: &[u8] = include_bytes!(r#"../libs\arm64/WDSCLIENTAPI.lib"#);
static ARM64_WDSMC: &[u8] = include_bytes!(r#"../libs\arm64/WDSMC.lib"#);
static ARM64_WDSPXE: &[u8] = include_bytes!(r#"../libs\arm64/WDSPXE.lib"#);
static ARM64_WDSTPTC: &[u8] = include_bytes!(r#"../libs\arm64/WDSTPTC.lib"#);
static ARM64_WEBAUTHN: &[u8] = include_bytes!(r#"../libs\arm64/webauthn.lib"#);
static ARM64_WEBSERVICES: &[u8] = include_bytes!(r#"../libs\arm64/webservices.lib"#);
static ARM64_WEBSOCKET: &[u8] = include_bytes!(r#"../libs\arm64/websocket.lib"#);
static ARM64_WECAPI: &[u8] = include_bytes!(r#"../libs\arm64/WecApi.lib"#);
static ARM64_WER: &[u8] = include_bytes!(r#"../libs\arm64/wer.lib"#);
static ARM64_WEVTAPI: &[u8] = include_bytes!(r#"../libs\arm64/wevtapi.lib"#);
static ARM64_WINBIO: &[u8] = include_bytes!(r#"../libs\arm64/winbio.lib"#);
static ARM64_WINDOWS_AI_MACHINELEARNING: &[u8] = include_bytes!(r#"../libs\arm64/windows.ai.machinelearning.lib"#);
static ARM64_WINDOWS_DATA_PDF: &[u8] = include_bytes!(r#"../libs\arm64/Windows.Data.Pdf.lib"#);
static ARM64_WINDOWS_NETWORKING: &[u8] = include_bytes!(r#"../libs\arm64/Windows.Networking.lib"#);
static ARM64_WINDOWS_UI_XAML: &[u8] = include_bytes!(r#"../libs\arm64/Windows.UI.Xaml.lib"#);
static ARM64_WINDOWSCODECS: &[u8] = include_bytes!(r#"../libs\arm64/WindowsCodecs.lib"#);
static ARM64_WINFAX: &[u8] = include_bytes!(r#"../libs\arm64/WINFAX.lib"#);
static ARM64_WINHTTP: &[u8] = include_bytes!(r#"../libs\arm64/WINHTTP.lib"#);
static ARM64_WINHVEMULATION: &[u8] = include_bytes!(r#"../libs\arm64/WinHvEmulation.lib"#);
static ARM64_WINHVPLATFORM: &[u8] = include_bytes!(r#"../libs\arm64/WinHvPlatform.lib"#);
static ARM64_WININET: &[u8] = include_bytes!(r#"../libs\arm64/WININET.lib"#);
static ARM64_WINML: &[u8] = include_bytes!(r#"../libs\arm64/winml.lib"#);
static ARM64_WINMM: &[u8] = include_bytes!(r#"../libs\arm64/WINMM.lib"#);
static ARM64_WINSCARD: &[u8] = include_bytes!(r#"../libs\arm64/WinSCard.lib"#);
static ARM64_WINSPOOL: &[u8] = include_bytes!(r#"../libs\arm64/WINSPOOL.lib"#);
static ARM64_WINTRUST: &[u8] = include_bytes!(r#"../libs\arm64/WINTRUST.lib"#);
static ARM64_WINUSB: &[u8] = include_bytes!(r#"../libs\arm64/WINUSB.lib"#);
static ARM64_WLANAPI: &[u8] = include_bytes!(r#"../libs\arm64/wlanapi.lib"#);
static ARM64_WLANUI: &[u8] = include_bytes!(r#"../libs\arm64/wlanui.lib"#);
static ARM64_WLDAP32: &[u8] = include_bytes!(r#"../libs\arm64/WLDAP32.lib"#);
static ARM64_WLDP: &[u8] = include_bytes!(r#"../libs\arm64/Wldp.lib"#);
static ARM64_WMVCORE: &[u8] = include_bytes!(r#"../libs\arm64/WMVCore.lib"#);
static ARM64_WNVAPI: &[u8] = include_bytes!(r#"../libs\arm64/wnvapi.lib"#);
static ARM64_WOFUTIL: &[u8] = include_bytes!(r#"../libs\arm64/WOFUTIL.lib"#);
static ARM64_WS2_32: &[u8] = include_bytes!(r#"../libs\arm64/WS2_32.lib"#);
static ARM64_WSCAPI: &[u8] = include_bytes!(r#"../libs\arm64/WSCAPI.lib"#);
static ARM64_WSCLIENT: &[u8] = include_bytes!(r#"../libs\arm64/WSClient.lib"#);
static ARM64_WSDAPI: &[u8] = include_bytes!(r#"../libs\arm64/wsdapi.lib"#);
static ARM64_WSMSVC: &[u8] = include_bytes!(r#"../libs\arm64/WsmSvc.lib"#);
static ARM64_WSNMP32: &[u8] = include_bytes!(r#"../libs\arm64/wsnmp32.lib"#);
static ARM64_WTSAPI32: &[u8] = include_bytes!(r#"../libs\arm64/WTSAPI32.lib"#);
static ARM64_XAUDIO2_8: &[u8] = include_bytes!(r#"../libs\arm64/XAudio2_8.lib"#);
static ARM64_XINPUTUAP: &[u8] = include_bytes!(r#"../libs\arm64/XINPUTUAP.lib"#);
static ARM64_XMLLITE: &[u8] = include_bytes!(r#"../libs\arm64/XmlLite.lib"#);
