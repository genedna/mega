#[inline]
pub unsafe fn AddClusterGroupDependency<P0, P1>(hdependentgroup: P0, hprovidergroup: P1) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn AddClusterGroupDependency(hdependentgroup : HGROUP, hprovidergroup : HGROUP) -> u32);
    AddClusterGroupDependency(hdependentgroup.param().abi(), hprovidergroup.param().abi())
}
#[inline]
pub unsafe fn AddClusterGroupDependencyEx<P0, P1, P2>(hdependentgroup: P0, hprovidergroup: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HGROUP>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn AddClusterGroupDependencyEx(hdependentgroup : HGROUP, hprovidergroup : HGROUP, lpszreason : windows_core::PCWSTR) -> u32);
    AddClusterGroupDependencyEx(hdependentgroup.param().abi(), hprovidergroup.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn AddClusterGroupSetDependency<P0, P1>(hdependentgroupset: P0, hprovidergroupset: P1) -> u32
where
    P0: windows_core::Param<HGROUPSET>,
    P1: windows_core::Param<HGROUPSET>,
{
    windows_targets::link!("clusapi.dll" "system" fn AddClusterGroupSetDependency(hdependentgroupset : HGROUPSET, hprovidergroupset : HGROUPSET) -> u32);
    AddClusterGroupSetDependency(hdependentgroupset.param().abi(), hprovidergroupset.param().abi())
}
#[inline]
pub unsafe fn AddClusterGroupSetDependencyEx<P0, P1, P2>(hdependentgroupset: P0, hprovidergroupset: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HGROUPSET>,
    P1: windows_core::Param<HGROUPSET>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn AddClusterGroupSetDependencyEx(hdependentgroupset : HGROUPSET, hprovidergroupset : HGROUPSET, lpszreason : windows_core::PCWSTR) -> u32);
    AddClusterGroupSetDependencyEx(hdependentgroupset.param().abi(), hprovidergroupset.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn AddClusterGroupToGroupSetDependency<P0, P1>(hdependentgroup: P0, hprovidergroupset: P1) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HGROUPSET>,
{
    windows_targets::link!("clusapi.dll" "system" fn AddClusterGroupToGroupSetDependency(hdependentgroup : HGROUP, hprovidergroupset : HGROUPSET) -> u32);
    AddClusterGroupToGroupSetDependency(hdependentgroup.param().abi(), hprovidergroupset.param().abi())
}
#[inline]
pub unsafe fn AddClusterGroupToGroupSetDependencyEx<P0, P1, P2>(hdependentgroup: P0, hprovidergroupset: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HGROUPSET>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn AddClusterGroupToGroupSetDependencyEx(hdependentgroup : HGROUP, hprovidergroupset : HGROUPSET, lpszreason : windows_core::PCWSTR) -> u32);
    AddClusterGroupToGroupSetDependencyEx(hdependentgroup.param().abi(), hprovidergroupset.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn AddClusterNode<P0, P1>(hcluster: P0, lpsznodename: P1, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>) -> HNODE
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn AddClusterNode(hcluster : HCLUSTER, lpsznodename : windows_core::PCWSTR, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void) -> HNODE);
    AddClusterNode(hcluster.param().abi(), lpsznodename.param().abi(), pfnprogresscallback, core::mem::transmute(pvcallbackarg.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn AddClusterNodeEx<P0, P1>(hcluster: P0, lpsznodename: P1, dwflags: u32, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>) -> HNODE
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn AddClusterNodeEx(hcluster : HCLUSTER, lpsznodename : windows_core::PCWSTR, dwflags : u32, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void) -> HNODE);
    AddClusterNodeEx(hcluster.param().abi(), lpsznodename.param().abi(), dwflags, pfnprogresscallback, core::mem::transmute(pvcallbackarg.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn AddClusterResourceDependency<P0, P1>(hresource: P0, hdependson: P1) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn AddClusterResourceDependency(hresource : HRESOURCE, hdependson : HRESOURCE) -> u32);
    AddClusterResourceDependency(hresource.param().abi(), hdependson.param().abi())
}
#[inline]
pub unsafe fn AddClusterResourceDependencyEx<P0, P1, P2>(hresource: P0, hdependson: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HRESOURCE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn AddClusterResourceDependencyEx(hresource : HRESOURCE, hdependson : HRESOURCE, lpszreason : windows_core::PCWSTR) -> u32);
    AddClusterResourceDependencyEx(hresource.param().abi(), hdependson.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn AddClusterResourceNode<P0, P1>(hresource: P0, hnode: P1) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn AddClusterResourceNode(hresource : HRESOURCE, hnode : HNODE) -> u32);
    AddClusterResourceNode(hresource.param().abi(), hnode.param().abi())
}
#[inline]
pub unsafe fn AddClusterResourceNodeEx<P0, P1, P2>(hresource: P0, hnode: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HNODE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn AddClusterResourceNodeEx(hresource : HRESOURCE, hnode : HNODE, lpszreason : windows_core::PCWSTR) -> u32);
    AddClusterResourceNodeEx(hresource.param().abi(), hnode.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn AddClusterStorageNode<P0, P1, P2, P3>(hcluster: P0, lpsznodename: P1, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>, lpszclusterstoragenodedescription: P2, lpszclusterstoragenodelocation: P3) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn AddClusterStorageNode(hcluster : HCLUSTER, lpsznodename : windows_core::PCWSTR, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void, lpszclusterstoragenodedescription : windows_core::PCWSTR, lpszclusterstoragenodelocation : windows_core::PCWSTR) -> u32);
    AddClusterStorageNode(hcluster.param().abi(), lpsznodename.param().abi(), pfnprogresscallback, core::mem::transmute(pvcallbackarg.unwrap_or(std::ptr::null())), lpszclusterstoragenodedescription.param().abi(), lpszclusterstoragenodelocation.param().abi())
}
#[inline]
pub unsafe fn AddCrossClusterGroupSetDependency<P0, P1, P2>(hdependentgroupset: P0, lpremoteclustername: P1, lpremotegroupsetname: P2) -> u32
where
    P0: windows_core::Param<HGROUPSET>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn AddCrossClusterGroupSetDependency(hdependentgroupset : HGROUPSET, lpremoteclustername : windows_core::PCWSTR, lpremotegroupsetname : windows_core::PCWSTR) -> u32);
    AddCrossClusterGroupSetDependency(hdependentgroupset.param().abi(), lpremoteclustername.param().abi(), lpremotegroupsetname.param().abi())
}
#[inline]
pub unsafe fn AddResourceToClusterSharedVolumes<P0>(hresource: P0) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn AddResourceToClusterSharedVolumes(hresource : HRESOURCE) -> u32);
    AddResourceToClusterSharedVolumes(hresource.param().abi())
}
#[inline]
pub unsafe fn BackupClusterDatabase<P0, P1>(hcluster: P0, lpszpathname: P1) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn BackupClusterDatabase(hcluster : HCLUSTER, lpszpathname : windows_core::PCWSTR) -> u32);
    BackupClusterDatabase(hcluster.param().abi(), lpszpathname.param().abi())
}
#[inline]
pub unsafe fn CanResourceBeDependent<P0, P1>(hresource: P0, hresourcedependent: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn CanResourceBeDependent(hresource : HRESOURCE, hresourcedependent : HRESOURCE) -> super::super::Foundation:: BOOL);
    CanResourceBeDependent(hresource.param().abi(), hresourcedependent.param().abi())
}
#[inline]
pub unsafe fn CancelClusterGroupOperation<P0>(hgroup: P0, dwcancelflags_reserved: u32) -> u32
where
    P0: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn CancelClusterGroupOperation(hgroup : HGROUP, dwcancelflags_reserved : u32) -> u32);
    CancelClusterGroupOperation(hgroup.param().abi(), dwcancelflags_reserved)
}
#[inline]
pub unsafe fn ChangeClusterResourceGroup<P0, P1>(hresource: P0, hgroup: P1) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn ChangeClusterResourceGroup(hresource : HRESOURCE, hgroup : HGROUP) -> u32);
    ChangeClusterResourceGroup(hresource.param().abi(), hgroup.param().abi())
}
#[inline]
pub unsafe fn ChangeClusterResourceGroupEx<P0, P1>(hresource: P0, hgroup: P1, flags: u64) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn ChangeClusterResourceGroupEx(hresource : HRESOURCE, hgroup : HGROUP, flags : u64) -> u32);
    ChangeClusterResourceGroupEx(hresource.param().abi(), hgroup.param().abi(), flags)
}
#[inline]
pub unsafe fn ChangeClusterResourceGroupEx2<P0, P1, P2>(hresource: P0, hgroup: P1, flags: u64, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HGROUP>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ChangeClusterResourceGroupEx2(hresource : HRESOURCE, hgroup : HGROUP, flags : u64, lpszreason : windows_core::PCWSTR) -> u32);
    ChangeClusterResourceGroupEx2(hresource.param().abi(), hgroup.param().abi(), flags, lpszreason.param().abi())
}
#[inline]
pub unsafe fn CloseCluster<P0>(hcluster: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("clusapi.dll" "system" fn CloseCluster(hcluster : HCLUSTER) -> super::super::Foundation:: BOOL);
    CloseCluster(hcluster.param().abi())
}
#[inline]
pub unsafe fn CloseClusterCryptProvider<P0>(hcluscryptprovider: P0) -> u32
where
    P0: windows_core::Param<HCLUSCRYPTPROVIDER>,
{
    windows_targets::link!("resutils.dll" "system" fn CloseClusterCryptProvider(hcluscryptprovider : HCLUSCRYPTPROVIDER) -> u32);
    CloseClusterCryptProvider(hcluscryptprovider.param().abi())
}
#[inline]
pub unsafe fn CloseClusterGroup<P0>(hgroup: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn CloseClusterGroup(hgroup : HGROUP) -> super::super::Foundation:: BOOL);
    CloseClusterGroup(hgroup.param().abi()).ok()
}
#[inline]
pub unsafe fn CloseClusterGroupSet<P0>(hgroupset: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HGROUPSET>,
{
    windows_targets::link!("clusapi.dll" "system" fn CloseClusterGroupSet(hgroupset : HGROUPSET) -> super::super::Foundation:: BOOL);
    CloseClusterGroupSet(hgroupset.param().abi()).ok()
}
#[inline]
pub unsafe fn CloseClusterNetInterface<P0>(hnetinterface: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HNETINTERFACE>,
{
    windows_targets::link!("clusapi.dll" "system" fn CloseClusterNetInterface(hnetinterface : HNETINTERFACE) -> super::super::Foundation:: BOOL);
    CloseClusterNetInterface(hnetinterface.param().abi()).ok()
}
#[inline]
pub unsafe fn CloseClusterNetwork<P0>(hnetwork: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HNETWORK>,
{
    windows_targets::link!("clusapi.dll" "system" fn CloseClusterNetwork(hnetwork : HNETWORK) -> super::super::Foundation:: BOOL);
    CloseClusterNetwork(hnetwork.param().abi()).ok()
}
#[inline]
pub unsafe fn CloseClusterNode<P0>(hnode: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn CloseClusterNode(hnode : HNODE) -> super::super::Foundation:: BOOL);
    CloseClusterNode(hnode.param().abi()).ok()
}
#[inline]
pub unsafe fn CloseClusterNotifyPort<P0>(hchange: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HCHANGE>,
{
    windows_targets::link!("clusapi.dll" "system" fn CloseClusterNotifyPort(hchange : HCHANGE) -> super::super::Foundation:: BOOL);
    CloseClusterNotifyPort(hchange.param().abi())
}
#[inline]
pub unsafe fn CloseClusterResource<P0>(hresource: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn CloseClusterResource(hresource : HRESOURCE) -> super::super::Foundation:: BOOL);
    CloseClusterResource(hresource.param().abi()).ok()
}
#[inline]
pub unsafe fn ClusAddClusterHealthFault<P0>(hcluster: P0, failure: *const CLUSTER_HEALTH_FAULT, param2: u32) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("resutils.dll" "system" fn ClusAddClusterHealthFault(hcluster : HCLUSTER, failure : *const CLUSTER_HEALTH_FAULT, param2 : u32) -> u32);
    ClusAddClusterHealthFault(hcluster.param().abi(), failure, param2)
}
#[inline]
pub unsafe fn ClusGetClusterHealthFaults<P0>(hcluster: P0, objects: *mut CLUSTER_HEALTH_FAULT_ARRAY, flags: u32) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("resutils.dll" "system" fn ClusGetClusterHealthFaults(hcluster : HCLUSTER, objects : *mut CLUSTER_HEALTH_FAULT_ARRAY, flags : u32) -> u32);
    ClusGetClusterHealthFaults(hcluster.param().abi(), objects, flags)
}
#[inline]
pub unsafe fn ClusRemoveClusterHealthFault<P0, P1>(hcluster: P0, id: P1, flags: u32) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ClusRemoveClusterHealthFault(hcluster : HCLUSTER, id : windows_core::PCWSTR, flags : u32) -> u32);
    ClusRemoveClusterHealthFault(hcluster.param().abi(), id.param().abi(), flags)
}
#[inline]
pub unsafe fn ClusWorkerCheckTerminate(lpworker: *mut CLUS_WORKER) -> super::super::Foundation::BOOL {
    windows_targets::link!("resutils.dll" "system" fn ClusWorkerCheckTerminate(lpworker : *mut CLUS_WORKER) -> super::super::Foundation:: BOOL);
    ClusWorkerCheckTerminate(lpworker)
}
#[inline]
pub unsafe fn ClusWorkerCreate(lpworker: *mut CLUS_WORKER, lpstartaddress: PWORKER_START_ROUTINE, lpparameter: *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn ClusWorkerCreate(lpworker : *mut CLUS_WORKER, lpstartaddress : PWORKER_START_ROUTINE, lpparameter : *mut core::ffi::c_void) -> u32);
    ClusWorkerCreate(lpworker, lpstartaddress, lpparameter)
}
#[inline]
pub unsafe fn ClusWorkerTerminate(lpworker: *const CLUS_WORKER) {
    windows_targets::link!("resutils.dll" "system" fn ClusWorkerTerminate(lpworker : *const CLUS_WORKER));
    ClusWorkerTerminate(lpworker)
}
#[inline]
pub unsafe fn ClusWorkerTerminateEx<P0>(clusworker: *mut CLUS_WORKER, timeoutinmilliseconds: u32, waitonly: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("resutils.dll" "system" fn ClusWorkerTerminateEx(clusworker : *mut CLUS_WORKER, timeoutinmilliseconds : u32, waitonly : super::super::Foundation:: BOOL) -> u32);
    ClusWorkerTerminateEx(clusworker, timeoutinmilliseconds, waitonly.param().abi())
}
#[inline]
pub unsafe fn ClusWorkersTerminate<P0>(clusworkers: &mut [*mut CLUS_WORKER], timeoutinmilliseconds: u32, waitonly: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("resutils.dll" "system" fn ClusWorkersTerminate(clusworkers : *mut *mut CLUS_WORKER, clusworkerscount : usize, timeoutinmilliseconds : u32, waitonly : super::super::Foundation:: BOOL) -> u32);
    ClusWorkersTerminate(core::mem::transmute(clusworkers.as_ptr()), clusworkers.len().try_into().unwrap(), timeoutinmilliseconds, waitonly.param().abi())
}
#[inline]
pub unsafe fn ClusapiSetReasonHandler(lphandler: *const CLUSAPI_REASON_HANDLER) -> *mut CLUSAPI_REASON_HANDLER {
    windows_targets::link!("clusapi.dll" "system" fn ClusapiSetReasonHandler(lphandler : *const CLUSAPI_REASON_HANDLER) -> *mut CLUSAPI_REASON_HANDLER);
    ClusapiSetReasonHandler(lphandler)
}
#[inline]
pub unsafe fn ClusterAddGroupToAffinityRule<P0, P1, P2>(hcluster: P0, rulename: P1, hgroup: P2) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterAddGroupToAffinityRule(hcluster : HCLUSTER, rulename : windows_core::PCWSTR, hgroup : HGROUP) -> u32);
    ClusterAddGroupToAffinityRule(hcluster.param().abi(), rulename.param().abi(), hgroup.param().abi())
}
#[inline]
pub unsafe fn ClusterAddGroupToGroupSet<P0, P1>(hgroupset: P0, hgroup: P1) -> u32
where
    P0: windows_core::Param<HGROUPSET>,
    P1: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterAddGroupToGroupSet(hgroupset : HGROUPSET, hgroup : HGROUP) -> u32);
    ClusterAddGroupToGroupSet(hgroupset.param().abi(), hgroup.param().abi())
}
#[inline]
pub unsafe fn ClusterAddGroupToGroupSetWithDomains<P0, P1>(hgroupset: P0, hgroup: P1, faultdomain: u32, updatedomain: u32) -> u32
where
    P0: windows_core::Param<HGROUPSET>,
    P1: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterAddGroupToGroupSetWithDomains(hgroupset : HGROUPSET, hgroup : HGROUP, faultdomain : u32, updatedomain : u32) -> u32);
    ClusterAddGroupToGroupSetWithDomains(hgroupset.param().abi(), hgroup.param().abi(), faultdomain, updatedomain)
}
#[inline]
pub unsafe fn ClusterAddGroupToGroupSetWithDomainsEx<P0, P1, P2>(hgroupset: P0, hgroup: P1, faultdomain: u32, updatedomain: u32, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HGROUPSET>,
    P1: windows_core::Param<HGROUP>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterAddGroupToGroupSetWithDomainsEx(hgroupset : HGROUPSET, hgroup : HGROUP, faultdomain : u32, updatedomain : u32, lpszreason : windows_core::PCWSTR) -> u32);
    ClusterAddGroupToGroupSetWithDomainsEx(hgroupset.param().abi(), hgroup.param().abi(), faultdomain, updatedomain, lpszreason.param().abi())
}
#[inline]
pub unsafe fn ClusterAffinityRuleControl<P0, P1, P2>(hcluster: P0, affinityrulename: P1, hhostnode: P2, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterAffinityRuleControl(hcluster : HCLUSTER, affinityrulename : windows_core::PCWSTR, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    ClusterAffinityRuleControl(hcluster.param().abi(), affinityrulename.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), cbinbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), cboutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ClusterClearBackupStateForSharedVolume<P0>(lpszvolumepathname: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ClusterClearBackupStateForSharedVolume(lpszvolumepathname : windows_core::PCWSTR) -> u32);
    ClusterClearBackupStateForSharedVolume(lpszvolumepathname.param().abi())
}
#[inline]
pub unsafe fn ClusterCloseEnum<P0>(henum: P0) -> u32
where
    P0: windows_core::Param<HCLUSENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterCloseEnum(henum : HCLUSENUM) -> u32);
    ClusterCloseEnum(henum.param().abi())
}
#[inline]
pub unsafe fn ClusterCloseEnumEx<P0>(hclusterenum: P0) -> u32
where
    P0: windows_core::Param<HCLUSENUMEX>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterCloseEnumEx(hclusterenum : HCLUSENUMEX) -> u32);
    ClusterCloseEnumEx(hclusterenum.param().abi())
}
#[inline]
pub unsafe fn ClusterControl<P0, P1>(hcluster: P0, hhostnode: P1, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterControl(hcluster : HCLUSTER, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    ClusterControl(hcluster.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ClusterControlEx<P0, P1, P2>(hcluster: P0, hhostnode: P1, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<HNODE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterControlEx(hcluster : HCLUSTER, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    ClusterControlEx(hcluster.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())), lpszreason.param().abi())
}
#[inline]
pub unsafe fn ClusterCreateAffinityRule<P0, P1>(hcluster: P0, rulename: P1, ruletype: CLUS_AFFINITY_RULE_TYPE) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterCreateAffinityRule(hcluster : HCLUSTER, rulename : windows_core::PCWSTR, ruletype : CLUS_AFFINITY_RULE_TYPE) -> u32);
    ClusterCreateAffinityRule(hcluster.param().abi(), rulename.param().abi(), ruletype)
}
#[inline]
pub unsafe fn ClusterDecrypt<P0>(hcluscryptprovider: P0, pcryptinput: *const u8, cbcryptinput: u32, ppcryptoutput: *mut *mut u8, pcbcryptoutput: *mut u32) -> u32
where
    P0: windows_core::Param<HCLUSCRYPTPROVIDER>,
{
    windows_targets::link!("resutils.dll" "system" fn ClusterDecrypt(hcluscryptprovider : HCLUSCRYPTPROVIDER, pcryptinput : *const u8, cbcryptinput : u32, ppcryptoutput : *mut *mut u8, pcbcryptoutput : *mut u32) -> u32);
    ClusterDecrypt(hcluscryptprovider.param().abi(), pcryptinput, cbcryptinput, ppcryptoutput, pcbcryptoutput)
}
#[inline]
pub unsafe fn ClusterEncrypt<P0>(hcluscryptprovider: P0, pdata: &[u8], ppdata: *mut *mut u8, pcbdata: *mut u32) -> u32
where
    P0: windows_core::Param<HCLUSCRYPTPROVIDER>,
{
    windows_targets::link!("resutils.dll" "system" fn ClusterEncrypt(hcluscryptprovider : HCLUSCRYPTPROVIDER, pdata : *const u8, cbdata : u32, ppdata : *mut *mut u8, pcbdata : *mut u32) -> u32);
    ClusterEncrypt(hcluscryptprovider.param().abi(), core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), ppdata, pcbdata)
}
#[inline]
pub unsafe fn ClusterEnum<P0>(henum: P0, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32
where
    P0: windows_core::Param<HCLUSENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterEnum(henum : HCLUSENUM, dwindex : u32, lpdwtype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    ClusterEnum(henum.param().abi(), dwindex, lpdwtype, core::mem::transmute(lpszname), lpcchname)
}
#[inline]
pub unsafe fn ClusterEnumEx<P0>(hclusterenum: P0, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32
where
    P0: windows_core::Param<HCLUSENUMEX>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterEnumEx(hclusterenum : HCLUSENUMEX, dwindex : u32, pitem : *mut CLUSTER_ENUM_ITEM, cbitem : *mut u32) -> u32);
    ClusterEnumEx(hclusterenum.param().abi(), dwindex, pitem, cbitem)
}
#[inline]
pub unsafe fn ClusterGetEnumCount<P0>(henum: P0) -> u32
where
    P0: windows_core::Param<HCLUSENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGetEnumCount(henum : HCLUSENUM) -> u32);
    ClusterGetEnumCount(henum.param().abi())
}
#[inline]
pub unsafe fn ClusterGetEnumCountEx<P0>(hclusterenum: P0) -> u32
where
    P0: windows_core::Param<HCLUSENUMEX>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGetEnumCountEx(hclusterenum : HCLUSENUMEX) -> u32);
    ClusterGetEnumCountEx(hclusterenum.param().abi())
}
#[inline]
pub unsafe fn ClusterGetVolumeNameForVolumeMountPoint<P0>(lpszvolumemountpoint: P0, lpszvolumename: windows_core::PWSTR, cchbufferlength: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ClusterGetVolumeNameForVolumeMountPoint(lpszvolumemountpoint : windows_core::PCWSTR, lpszvolumename : windows_core::PWSTR, cchbufferlength : u32) -> super::super::Foundation:: BOOL);
    ClusterGetVolumeNameForVolumeMountPoint(lpszvolumemountpoint.param().abi(), core::mem::transmute(lpszvolumename), cchbufferlength).ok()
}
#[inline]
pub unsafe fn ClusterGetVolumePathName<P0>(lpszfilename: P0, lpszvolumepathname: windows_core::PWSTR, cchbufferlength: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ClusterGetVolumePathName(lpszfilename : windows_core::PCWSTR, lpszvolumepathname : windows_core::PWSTR, cchbufferlength : u32) -> super::super::Foundation:: BOOL);
    ClusterGetVolumePathName(lpszfilename.param().abi(), core::mem::transmute(lpszvolumepathname), cchbufferlength).ok()
}
#[inline]
pub unsafe fn ClusterGroupCloseEnum<P0>(hgroupenum: P0) -> u32
where
    P0: windows_core::Param<HGROUPENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGroupCloseEnum(hgroupenum : HGROUPENUM) -> u32);
    ClusterGroupCloseEnum(hgroupenum.param().abi())
}
#[inline]
pub unsafe fn ClusterGroupCloseEnumEx<P0>(hgroupenumex: P0) -> u32
where
    P0: windows_core::Param<HGROUPENUMEX>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGroupCloseEnumEx(hgroupenumex : HGROUPENUMEX) -> u32);
    ClusterGroupCloseEnumEx(hgroupenumex.param().abi())
}
#[inline]
pub unsafe fn ClusterGroupControl<P0, P1>(hgroup: P0, hhostnode: P1, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGroupControl(hgroup : HGROUP, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    ClusterGroupControl(hgroup.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ClusterGroupControlEx<P0, P1, P2>(hgroup: P0, hhostnode: P1, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HNODE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGroupControlEx(hgroup : HGROUP, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    ClusterGroupControlEx(hgroup.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())), lpszreason.param().abi())
}
#[inline]
pub unsafe fn ClusterGroupEnum<P0>(hgroupenum: P0, dwindex: u32, lpdwtype: *mut u32, lpszresourcename: windows_core::PWSTR, lpcchname: *mut u32) -> u32
where
    P0: windows_core::Param<HGROUPENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGroupEnum(hgroupenum : HGROUPENUM, dwindex : u32, lpdwtype : *mut u32, lpszresourcename : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    ClusterGroupEnum(hgroupenum.param().abi(), dwindex, lpdwtype, core::mem::transmute(lpszresourcename), lpcchname)
}
#[inline]
pub unsafe fn ClusterGroupEnumEx<P0>(hgroupenumex: P0, dwindex: u32, pitem: *mut CLUSTER_GROUP_ENUM_ITEM, cbitem: *mut u32) -> u32
where
    P0: windows_core::Param<HGROUPENUMEX>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGroupEnumEx(hgroupenumex : HGROUPENUMEX, dwindex : u32, pitem : *mut CLUSTER_GROUP_ENUM_ITEM, cbitem : *mut u32) -> u32);
    ClusterGroupEnumEx(hgroupenumex.param().abi(), dwindex, pitem, cbitem)
}
#[inline]
pub unsafe fn ClusterGroupGetEnumCount<P0>(hgroupenum: P0) -> u32
where
    P0: windows_core::Param<HGROUPENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGroupGetEnumCount(hgroupenum : HGROUPENUM) -> u32);
    ClusterGroupGetEnumCount(hgroupenum.param().abi())
}
#[inline]
pub unsafe fn ClusterGroupGetEnumCountEx<P0>(hgroupenumex: P0) -> u32
where
    P0: windows_core::Param<HGROUPENUMEX>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGroupGetEnumCountEx(hgroupenumex : HGROUPENUMEX) -> u32);
    ClusterGroupGetEnumCountEx(hgroupenumex.param().abi())
}
#[inline]
pub unsafe fn ClusterGroupOpenEnum<P0>(hgroup: P0, dwtype: u32) -> HGROUPENUM
where
    P0: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGroupOpenEnum(hgroup : HGROUP, dwtype : u32) -> HGROUPENUM);
    ClusterGroupOpenEnum(hgroup.param().abi(), dwtype)
}
#[inline]
pub unsafe fn ClusterGroupOpenEnumEx<P0, P1, P2>(hcluster: P0, lpszproperties: P1, cbproperties: u32, lpszroproperties: P2, cbroproperties: u32, dwflags: u32) -> HGROUPENUMEX
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGroupOpenEnumEx(hcluster : HCLUSTER, lpszproperties : windows_core::PCWSTR, cbproperties : u32, lpszroproperties : windows_core::PCWSTR, cbroproperties : u32, dwflags : u32) -> HGROUPENUMEX);
    ClusterGroupOpenEnumEx(hcluster.param().abi(), lpszproperties.param().abi(), cbproperties, lpszroproperties.param().abi(), cbroproperties, dwflags)
}
#[inline]
pub unsafe fn ClusterGroupSetCloseEnum<P0>(hgroupsetenum: P0) -> u32
where
    P0: windows_core::Param<HGROUPSETENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGroupSetCloseEnum(hgroupsetenum : HGROUPSETENUM) -> u32);
    ClusterGroupSetCloseEnum(hgroupsetenum.param().abi())
}
#[inline]
pub unsafe fn ClusterGroupSetControl<P0, P1>(hgroupset: P0, hhostnode: P1, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<HGROUPSET>,
    P1: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGroupSetControl(hgroupset : HGROUPSET, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    ClusterGroupSetControl(hgroupset.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), cbinbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), cboutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ClusterGroupSetControlEx<P0, P1, P2>(hgroupset: P0, hhostnode: P1, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HGROUPSET>,
    P1: windows_core::Param<HNODE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGroupSetControlEx(hgroupset : HGROUPSET, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    ClusterGroupSetControlEx(hgroupset.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), cbinbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), cboutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())), lpszreason.param().abi())
}
#[inline]
pub unsafe fn ClusterGroupSetEnum<P0>(hgroupsetenum: P0, dwindex: u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32
where
    P0: windows_core::Param<HGROUPSETENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGroupSetEnum(hgroupsetenum : HGROUPSETENUM, dwindex : u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    ClusterGroupSetEnum(hgroupsetenum.param().abi(), dwindex, core::mem::transmute(lpszname), lpcchname)
}
#[inline]
pub unsafe fn ClusterGroupSetGetEnumCount<P0>(hgroupsetenum: P0) -> u32
where
    P0: windows_core::Param<HGROUPSETENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGroupSetGetEnumCount(hgroupsetenum : HGROUPSETENUM) -> u32);
    ClusterGroupSetGetEnumCount(hgroupsetenum.param().abi())
}
#[inline]
pub unsafe fn ClusterGroupSetOpenEnum<P0>(hcluster: P0) -> HGROUPSETENUM
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterGroupSetOpenEnum(hcluster : HCLUSTER) -> HGROUPSETENUM);
    ClusterGroupSetOpenEnum(hcluster.param().abi())
}
#[inline]
pub unsafe fn ClusterIsPathOnSharedVolume<P0>(lpszpathname: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ClusterIsPathOnSharedVolume(lpszpathname : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    ClusterIsPathOnSharedVolume(lpszpathname.param().abi())
}
#[inline]
pub unsafe fn ClusterNetInterfaceCloseEnum<P0>(hnetinterfaceenum: P0) -> u32
where
    P0: windows_core::Param<HNETINTERFACEENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNetInterfaceCloseEnum(hnetinterfaceenum : HNETINTERFACEENUM) -> u32);
    ClusterNetInterfaceCloseEnum(hnetinterfaceenum.param().abi())
}
#[inline]
pub unsafe fn ClusterNetInterfaceControl<P0, P1>(hnetinterface: P0, hhostnode: P1, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<HNETINTERFACE>,
    P1: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNetInterfaceControl(hnetinterface : HNETINTERFACE, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    ClusterNetInterfaceControl(hnetinterface.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ClusterNetInterfaceControlEx<P0, P1, P2>(hnetinterface: P0, hhostnode: P1, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HNETINTERFACE>,
    P1: windows_core::Param<HNODE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNetInterfaceControlEx(hnetinterface : HNETINTERFACE, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    ClusterNetInterfaceControlEx(hnetinterface.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())), lpszreason.param().abi())
}
#[inline]
pub unsafe fn ClusterNetInterfaceEnum<P0>(hnetinterfaceenum: P0, dwindex: u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32
where
    P0: windows_core::Param<HNETINTERFACEENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNetInterfaceEnum(hnetinterfaceenum : HNETINTERFACEENUM, dwindex : u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    ClusterNetInterfaceEnum(hnetinterfaceenum.param().abi(), dwindex, core::mem::transmute(lpszname), lpcchname)
}
#[inline]
pub unsafe fn ClusterNetInterfaceOpenEnum<P0, P1, P2>(hcluster: P0, lpsznodename: P1, lpsznetworkname: P2) -> HNETINTERFACEENUM
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNetInterfaceOpenEnum(hcluster : HCLUSTER, lpsznodename : windows_core::PCWSTR, lpsznetworkname : windows_core::PCWSTR) -> HNETINTERFACEENUM);
    ClusterNetInterfaceOpenEnum(hcluster.param().abi(), lpsznodename.param().abi(), lpsznetworkname.param().abi())
}
#[inline]
pub unsafe fn ClusterNetworkCloseEnum<P0>(hnetworkenum: P0) -> u32
where
    P0: windows_core::Param<HNETWORKENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNetworkCloseEnum(hnetworkenum : HNETWORKENUM) -> u32);
    ClusterNetworkCloseEnum(hnetworkenum.param().abi())
}
#[inline]
pub unsafe fn ClusterNetworkControl<P0, P1>(hnetwork: P0, hhostnode: P1, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<HNETWORK>,
    P1: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNetworkControl(hnetwork : HNETWORK, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    ClusterNetworkControl(hnetwork.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ClusterNetworkControlEx<P0, P1, P2>(hnetwork: P0, hhostnode: P1, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HNETWORK>,
    P1: windows_core::Param<HNODE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNetworkControlEx(hnetwork : HNETWORK, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    ClusterNetworkControlEx(hnetwork.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())), lpszreason.param().abi())
}
#[inline]
pub unsafe fn ClusterNetworkEnum<P0>(hnetworkenum: P0, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32
where
    P0: windows_core::Param<HNETWORKENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNetworkEnum(hnetworkenum : HNETWORKENUM, dwindex : u32, lpdwtype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    ClusterNetworkEnum(hnetworkenum.param().abi(), dwindex, lpdwtype, core::mem::transmute(lpszname), lpcchname)
}
#[inline]
pub unsafe fn ClusterNetworkGetEnumCount<P0>(hnetworkenum: P0) -> u32
where
    P0: windows_core::Param<HNETWORKENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNetworkGetEnumCount(hnetworkenum : HNETWORKENUM) -> u32);
    ClusterNetworkGetEnumCount(hnetworkenum.param().abi())
}
#[inline]
pub unsafe fn ClusterNetworkOpenEnum<P0>(hnetwork: P0, dwtype: u32) -> HNETWORKENUM
where
    P0: windows_core::Param<HNETWORK>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNetworkOpenEnum(hnetwork : HNETWORK, dwtype : u32) -> HNETWORKENUM);
    ClusterNetworkOpenEnum(hnetwork.param().abi(), dwtype)
}
#[inline]
pub unsafe fn ClusterNodeCloseEnum<P0>(hnodeenum: P0) -> u32
where
    P0: windows_core::Param<HNODEENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNodeCloseEnum(hnodeenum : HNODEENUM) -> u32);
    ClusterNodeCloseEnum(hnodeenum.param().abi())
}
#[inline]
pub unsafe fn ClusterNodeCloseEnumEx<P0>(hnodeenum: P0) -> u32
where
    P0: windows_core::Param<HNODEENUMEX>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNodeCloseEnumEx(hnodeenum : HNODEENUMEX) -> u32);
    ClusterNodeCloseEnumEx(hnodeenum.param().abi())
}
#[inline]
pub unsafe fn ClusterNodeControl<P0, P1>(hnode: P0, hhostnode: P1, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<HNODE>,
    P1: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNodeControl(hnode : HNODE, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    ClusterNodeControl(hnode.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ClusterNodeControlEx<P0, P1, P2>(hnode: P0, hhostnode: P1, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HNODE>,
    P1: windows_core::Param<HNODE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNodeControlEx(hnode : HNODE, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    ClusterNodeControlEx(hnode.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())), lpszreason.param().abi())
}
#[inline]
pub unsafe fn ClusterNodeEnum<P0>(hnodeenum: P0, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32
where
    P0: windows_core::Param<HNODEENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNodeEnum(hnodeenum : HNODEENUM, dwindex : u32, lpdwtype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    ClusterNodeEnum(hnodeenum.param().abi(), dwindex, lpdwtype, core::mem::transmute(lpszname), lpcchname)
}
#[inline]
pub unsafe fn ClusterNodeEnumEx<P0>(hnodeenum: P0, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32
where
    P0: windows_core::Param<HNODEENUMEX>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNodeEnumEx(hnodeenum : HNODEENUMEX, dwindex : u32, pitem : *mut CLUSTER_ENUM_ITEM, cbitem : *mut u32) -> u32);
    ClusterNodeEnumEx(hnodeenum.param().abi(), dwindex, pitem, cbitem)
}
#[inline]
pub unsafe fn ClusterNodeGetEnumCount<P0>(hnodeenum: P0) -> u32
where
    P0: windows_core::Param<HNODEENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNodeGetEnumCount(hnodeenum : HNODEENUM) -> u32);
    ClusterNodeGetEnumCount(hnodeenum.param().abi())
}
#[inline]
pub unsafe fn ClusterNodeGetEnumCountEx<P0>(hnodeenum: P0) -> u32
where
    P0: windows_core::Param<HNODEENUMEX>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNodeGetEnumCountEx(hnodeenum : HNODEENUMEX) -> u32);
    ClusterNodeGetEnumCountEx(hnodeenum.param().abi())
}
#[inline]
pub unsafe fn ClusterNodeOpenEnum<P0>(hnode: P0, dwtype: u32) -> HNODEENUM
where
    P0: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNodeOpenEnum(hnode : HNODE, dwtype : u32) -> HNODEENUM);
    ClusterNodeOpenEnum(hnode.param().abi(), dwtype)
}
#[inline]
pub unsafe fn ClusterNodeOpenEnumEx<P0>(hnode: P0, dwtype: u32, poptions: Option<*const core::ffi::c_void>) -> HNODEENUMEX
where
    P0: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNodeOpenEnumEx(hnode : HNODE, dwtype : u32, poptions : *const core::ffi::c_void) -> HNODEENUMEX);
    ClusterNodeOpenEnumEx(hnode.param().abi(), dwtype, core::mem::transmute(poptions.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn ClusterNodeReplacement<P0, P1, P2>(hcluster: P0, lpsznodenamecurrent: P1, lpsznodenamenew: P2) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterNodeReplacement(hcluster : HCLUSTER, lpsznodenamecurrent : windows_core::PCWSTR, lpsznodenamenew : windows_core::PCWSTR) -> u32);
    ClusterNodeReplacement(hcluster.param().abi(), lpsznodenamecurrent.param().abi(), lpsznodenamenew.param().abi())
}
#[inline]
pub unsafe fn ClusterOpenEnum<P0>(hcluster: P0, dwtype: u32) -> HCLUSENUM
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterOpenEnum(hcluster : HCLUSTER, dwtype : u32) -> HCLUSENUM);
    ClusterOpenEnum(hcluster.param().abi(), dwtype)
}
#[inline]
pub unsafe fn ClusterOpenEnumEx<P0>(hcluster: P0, dwtype: u32, poptions: Option<*const core::ffi::c_void>) -> HCLUSENUMEX
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterOpenEnumEx(hcluster : HCLUSTER, dwtype : u32, poptions : *const core::ffi::c_void) -> HCLUSENUMEX);
    ClusterOpenEnumEx(hcluster.param().abi(), dwtype, core::mem::transmute(poptions.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn ClusterPrepareSharedVolumeForBackup<P0>(lpszfilename: P0, lpszvolumepathname: windows_core::PWSTR, lpcchvolumepathname: *mut u32, lpszvolumename: windows_core::PWSTR, lpcchvolumename: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ClusterPrepareSharedVolumeForBackup(lpszfilename : windows_core::PCWSTR, lpszvolumepathname : windows_core::PWSTR, lpcchvolumepathname : *mut u32, lpszvolumename : windows_core::PWSTR, lpcchvolumename : *mut u32) -> u32);
    ClusterPrepareSharedVolumeForBackup(lpszfilename.param().abi(), core::mem::transmute(lpszvolumepathname), lpcchvolumepathname, core::mem::transmute(lpszvolumename), lpcchvolumename)
}
#[inline]
pub unsafe fn ClusterRegBatchAddCommand<P0, P1>(hregbatch: P0, dwcommand: CLUSTER_REG_COMMAND, wzname: P1, dwoptions: u32, lpdata: Option<*const core::ffi::c_void>, cbdata: u32) -> i32
where
    P0: windows_core::Param<HREGBATCH>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegBatchAddCommand(hregbatch : HREGBATCH, dwcommand : CLUSTER_REG_COMMAND, wzname : windows_core::PCWSTR, dwoptions : u32, lpdata : *const core::ffi::c_void, cbdata : u32) -> i32);
    ClusterRegBatchAddCommand(hregbatch.param().abi(), dwcommand, wzname.param().abi(), dwoptions, core::mem::transmute(lpdata.unwrap_or(std::ptr::null())), cbdata)
}
#[inline]
pub unsafe fn ClusterRegBatchCloseNotification<P0>(hbatchnotification: P0) -> i32
where
    P0: windows_core::Param<HREGBATCHNOTIFICATION>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegBatchCloseNotification(hbatchnotification : HREGBATCHNOTIFICATION) -> i32);
    ClusterRegBatchCloseNotification(hbatchnotification.param().abi())
}
#[inline]
pub unsafe fn ClusterRegBatchReadCommand<P0>(hbatchnotification: P0, pbatchcommand: *mut CLUSTER_BATCH_COMMAND) -> i32
where
    P0: windows_core::Param<HREGBATCHNOTIFICATION>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegBatchReadCommand(hbatchnotification : HREGBATCHNOTIFICATION, pbatchcommand : *mut CLUSTER_BATCH_COMMAND) -> i32);
    ClusterRegBatchReadCommand(hbatchnotification.param().abi(), pbatchcommand)
}
#[inline]
pub unsafe fn ClusterRegCloseBatch<P0, P1>(hregbatch: P0, bcommit: P1, failedcommandnumber: Option<*mut i32>) -> i32
where
    P0: windows_core::Param<HREGBATCH>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegCloseBatch(hregbatch : HREGBATCH, bcommit : super::super::Foundation:: BOOL, failedcommandnumber : *mut i32) -> i32);
    ClusterRegCloseBatch(hregbatch.param().abi(), bcommit.param().abi(), core::mem::transmute(failedcommandnumber.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ClusterRegCloseBatchEx<P0>(hregbatch: P0, flags: u32, failedcommandnumber: Option<*mut i32>) -> i32
where
    P0: windows_core::Param<HREGBATCH>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegCloseBatchEx(hregbatch : HREGBATCH, flags : u32, failedcommandnumber : *mut i32) -> i32);
    ClusterRegCloseBatchEx(hregbatch.param().abi(), flags, core::mem::transmute(failedcommandnumber.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ClusterRegCloseBatchNotifyPort<P0>(hbatchnotifyport: P0) -> i32
where
    P0: windows_core::Param<HREGBATCHPORT>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegCloseBatchNotifyPort(hbatchnotifyport : HREGBATCHPORT) -> i32);
    ClusterRegCloseBatchNotifyPort(hbatchnotifyport.param().abi())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegCloseKey<P0>(hkey: P0) -> i32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegCloseKey(hkey : super::super::System::Registry:: HKEY) -> i32);
    ClusterRegCloseKey(hkey.param().abi())
}
#[inline]
pub unsafe fn ClusterRegCloseReadBatch<P0>(hregreadbatch: P0, phregreadbatchreply: *mut HREGREADBATCHREPLY) -> i32
where
    P0: windows_core::Param<HREGREADBATCH>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegCloseReadBatch(hregreadbatch : HREGREADBATCH, phregreadbatchreply : *mut HREGREADBATCHREPLY) -> i32);
    ClusterRegCloseReadBatch(hregreadbatch.param().abi(), phregreadbatchreply)
}
#[inline]
pub unsafe fn ClusterRegCloseReadBatchEx<P0>(hregreadbatch: P0, flags: u32, phregreadbatchreply: *mut HREGREADBATCHREPLY) -> i32
where
    P0: windows_core::Param<HREGREADBATCH>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegCloseReadBatchEx(hregreadbatch : HREGREADBATCH, flags : u32, phregreadbatchreply : *mut HREGREADBATCHREPLY) -> i32);
    ClusterRegCloseReadBatchEx(hregreadbatch.param().abi(), flags, phregreadbatchreply)
}
#[inline]
pub unsafe fn ClusterRegCloseReadBatchReply<P0>(hregreadbatchreply: P0) -> i32
where
    P0: windows_core::Param<HREGREADBATCHREPLY>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegCloseReadBatchReply(hregreadbatchreply : HREGREADBATCHREPLY) -> i32);
    ClusterRegCloseReadBatchReply(hregreadbatchreply.param().abi())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegCreateBatch<P0>(hkey: P0, phregbatch: *mut HREGBATCH) -> i32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegCreateBatch(hkey : super::super::System::Registry:: HKEY, phregbatch : *mut HREGBATCH) -> i32);
    ClusterRegCreateBatch(hkey.param().abi(), phregbatch)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegCreateBatchNotifyPort<P0>(hkey: P0, phbatchnotifyport: *mut HREGBATCHPORT) -> i32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegCreateBatchNotifyPort(hkey : super::super::System::Registry:: HKEY, phbatchnotifyport : *mut HREGBATCHPORT) -> i32);
    ClusterRegCreateBatchNotifyPort(hkey.param().abi(), phbatchnotifyport)
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ClusterRegCreateKey<P0, P1>(hkey: P0, lpszsubkey: P1, dwoptions: u32, samdesired: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegCreateKey(hkey : super::super::System::Registry:: HKEY, lpszsubkey : windows_core::PCWSTR, dwoptions : u32, samdesired : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, phkresult : *mut super::super::System::Registry:: HKEY, lpdwdisposition : *mut u32) -> i32);
    ClusterRegCreateKey(hkey.param().abi(), lpszsubkey.param().abi(), dwoptions, samdesired, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), phkresult, core::mem::transmute(lpdwdisposition.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ClusterRegCreateKeyEx<P0, P1, P2>(hkey: P0, lpsubkey: P1, dwoptions: u32, samdesired: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: Option<*mut u32>, lpszreason: P2) -> i32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegCreateKeyEx(hkey : super::super::System::Registry:: HKEY, lpsubkey : windows_core::PCWSTR, dwoptions : u32, samdesired : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, phkresult : *mut super::super::System::Registry:: HKEY, lpdwdisposition : *mut u32, lpszreason : windows_core::PCWSTR) -> i32);
    ClusterRegCreateKeyEx(hkey.param().abi(), lpsubkey.param().abi(), dwoptions, samdesired, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), phkresult, core::mem::transmute(lpdwdisposition.unwrap_or(std::ptr::null_mut())), lpszreason.param().abi())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegCreateReadBatch<P0>(hkey: P0, phregreadbatch: *mut HREGREADBATCH) -> i32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegCreateReadBatch(hkey : super::super::System::Registry:: HKEY, phregreadbatch : *mut HREGREADBATCH) -> i32);
    ClusterRegCreateReadBatch(hkey.param().abi(), phregreadbatch)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegDeleteKey<P0, P1>(hkey: P0, lpszsubkey: P1) -> i32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegDeleteKey(hkey : super::super::System::Registry:: HKEY, lpszsubkey : windows_core::PCWSTR) -> i32);
    ClusterRegDeleteKey(hkey.param().abi(), lpszsubkey.param().abi())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegDeleteKeyEx<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpszreason: P2) -> i32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegDeleteKeyEx(hkey : super::super::System::Registry:: HKEY, lpsubkey : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> i32);
    ClusterRegDeleteKeyEx(hkey.param().abi(), lpsubkey.param().abi(), lpszreason.param().abi())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegDeleteValue<P0, P1>(hkey: P0, lpszvaluename: P1) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegDeleteValue(hkey : super::super::System::Registry:: HKEY, lpszvaluename : windows_core::PCWSTR) -> u32);
    ClusterRegDeleteValue(hkey.param().abi(), lpszvaluename.param().abi())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegDeleteValueEx<P0, P1, P2>(hkey: P0, lpszvaluename: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegDeleteValueEx(hkey : super::super::System::Registry:: HKEY, lpszvaluename : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    ClusterRegDeleteValueEx(hkey.param().abi(), lpszvaluename.param().abi(), lpszreason.param().abi())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegEnumKey<P0>(hkey: P0, dwindex: u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32, lpftlastwritetime: Option<*mut super::super::Foundation::FILETIME>) -> i32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegEnumKey(hkey : super::super::System::Registry:: HKEY, dwindex : u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32, lpftlastwritetime : *mut super::super::Foundation:: FILETIME) -> i32);
    ClusterRegEnumKey(hkey.param().abi(), dwindex, core::mem::transmute(lpszname), lpcchname, core::mem::transmute(lpftlastwritetime.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegEnumValue<P0>(hkey: P0, dwindex: u32, lpszvaluename: windows_core::PWSTR, lpcchvaluename: *mut u32, lpdwtype: Option<*mut u32>, lpdata: Option<*mut u8>, lpcbdata: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegEnumValue(hkey : super::super::System::Registry:: HKEY, dwindex : u32, lpszvaluename : windows_core::PWSTR, lpcchvaluename : *mut u32, lpdwtype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> u32);
    ClusterRegEnumValue(hkey.param().abi(), dwindex, core::mem::transmute(lpszvaluename), lpcchvaluename, core::mem::transmute(lpdwtype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpdata.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpcbdata.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ClusterRegGetBatchNotification<P0>(hbatchnotify: P0, phbatchnotification: *mut HREGBATCHNOTIFICATION) -> i32
where
    P0: windows_core::Param<HREGBATCHPORT>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegGetBatchNotification(hbatchnotify : HREGBATCHPORT, phbatchnotification : *mut HREGBATCHNOTIFICATION) -> i32);
    ClusterRegGetBatchNotification(hbatchnotify.param().abi(), phbatchnotification)
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ClusterRegGetKeySecurity<P0>(hkey: P0, requestedinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> i32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegGetKeySecurity(hkey : super::super::System::Registry:: HKEY, requestedinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> i32);
    ClusterRegGetKeySecurity(hkey.param().abi(), requestedinformation, psecuritydescriptor, lpcbsecuritydescriptor)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegOpenKey<P0, P1>(hkey: P0, lpszsubkey: P1, samdesired: u32, phkresult: *mut super::super::System::Registry::HKEY) -> i32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegOpenKey(hkey : super::super::System::Registry:: HKEY, lpszsubkey : windows_core::PCWSTR, samdesired : u32, phkresult : *mut super::super::System::Registry:: HKEY) -> i32);
    ClusterRegOpenKey(hkey.param().abi(), lpszsubkey.param().abi(), samdesired, phkresult)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegQueryInfoKey<P0>(hkey: P0, lpcsubkeys: *const u32, lpcchmaxsubkeylen: *const u32, lpcvalues: *const u32, lpcchmaxvaluenamelen: *const u32, lpcbmaxvaluelen: *const u32, lpcbsecuritydescriptor: *const u32, lpftlastwritetime: *const super::super::Foundation::FILETIME) -> i32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegQueryInfoKey(hkey : super::super::System::Registry:: HKEY, lpcsubkeys : *const u32, lpcchmaxsubkeylen : *const u32, lpcvalues : *const u32, lpcchmaxvaluenamelen : *const u32, lpcbmaxvaluelen : *const u32, lpcbsecuritydescriptor : *const u32, lpftlastwritetime : *const super::super::Foundation:: FILETIME) -> i32);
    ClusterRegQueryInfoKey(hkey.param().abi(), lpcsubkeys, lpcchmaxsubkeylen, lpcvalues, lpcchmaxvaluenamelen, lpcbmaxvaluelen, lpcbsecuritydescriptor, lpftlastwritetime)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegQueryValue<P0, P1>(hkey: P0, lpszvaluename: P1, lpdwvaluetype: Option<*mut u32>, lpdata: Option<*mut u8>, lpcbdata: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegQueryValue(hkey : super::super::System::Registry:: HKEY, lpszvaluename : windows_core::PCWSTR, lpdwvaluetype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> i32);
    ClusterRegQueryValue(hkey.param().abi(), lpszvaluename.param().abi(), core::mem::transmute(lpdwvaluetype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpdata.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpcbdata.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ClusterRegReadBatchAddCommand<P0, P1, P2>(hregreadbatch: P0, wzsubkeyname: P1, wzvaluename: P2) -> i32
where
    P0: windows_core::Param<HREGREADBATCH>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegReadBatchAddCommand(hregreadbatch : HREGREADBATCH, wzsubkeyname : windows_core::PCWSTR, wzvaluename : windows_core::PCWSTR) -> i32);
    ClusterRegReadBatchAddCommand(hregreadbatch.param().abi(), wzsubkeyname.param().abi(), wzvaluename.param().abi())
}
#[inline]
pub unsafe fn ClusterRegReadBatchReplyNextCommand<P0>(hregreadbatchreply: P0, pbatchcommand: *mut CLUSTER_READ_BATCH_COMMAND) -> i32
where
    P0: windows_core::Param<HREGREADBATCHREPLY>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegReadBatchReplyNextCommand(hregreadbatchreply : HREGREADBATCHREPLY, pbatchcommand : *mut CLUSTER_READ_BATCH_COMMAND) -> i32);
    ClusterRegReadBatchReplyNextCommand(hregreadbatchreply.param().abi(), pbatchcommand)
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ClusterRegSetKeySecurity<P0, P1>(hkey: P0, securityinformation: u32, psecuritydescriptor: P1) -> i32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegSetKeySecurity(hkey : super::super::System::Registry:: HKEY, securityinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> i32);
    ClusterRegSetKeySecurity(hkey.param().abi(), securityinformation, psecuritydescriptor.param().abi())
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ClusterRegSetKeySecurityEx<P0, P1, P2>(hkey: P0, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: P1, lpszreason: P2) -> i32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<super::super::Security::PSECURITY_DESCRIPTOR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegSetKeySecurityEx(hkey : super::super::System::Registry:: HKEY, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, lpszreason : windows_core::PCWSTR) -> i32);
    ClusterRegSetKeySecurityEx(hkey.param().abi(), securityinformation, psecuritydescriptor.param().abi(), lpszreason.param().abi())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegSetValue<P0, P1>(hkey: P0, lpszvaluename: P1, dwtype: u32, lpdata: *const u8, cbdata: u32) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegSetValue(hkey : super::super::System::Registry:: HKEY, lpszvaluename : windows_core::PCWSTR, dwtype : u32, lpdata : *const u8, cbdata : u32) -> u32);
    ClusterRegSetValue(hkey.param().abi(), lpszvaluename.param().abi(), dwtype, lpdata, cbdata)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegSetValueEx<P0, P1, P2>(hkey: P0, lpszvaluename: P1, dwtype: u32, lpdata: *const u8, cbdata: u32, lpszreason: P2) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegSetValueEx(hkey : super::super::System::Registry:: HKEY, lpszvaluename : windows_core::PCWSTR, dwtype : u32, lpdata : *const u8, cbdata : u32, lpszreason : windows_core::PCWSTR) -> u32);
    ClusterRegSetValueEx(hkey.param().abi(), lpszvaluename.param().abi(), dwtype, lpdata, cbdata, lpszreason.param().abi())
}
#[inline]
pub unsafe fn ClusterRegSyncDatabase<P0>(hcluster: P0, flags: u32) -> i32
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRegSyncDatabase(hcluster : HCLUSTER, flags : u32) -> i32);
    ClusterRegSyncDatabase(hcluster.param().abi(), flags)
}
#[inline]
pub unsafe fn ClusterRemoveAffinityRule<P0, P1>(hcluster: P0, rulename: P1) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRemoveAffinityRule(hcluster : HCLUSTER, rulename : windows_core::PCWSTR) -> u32);
    ClusterRemoveAffinityRule(hcluster.param().abi(), rulename.param().abi())
}
#[inline]
pub unsafe fn ClusterRemoveGroupFromAffinityRule<P0, P1, P2>(hcluster: P0, rulename: P1, hgroup: P2) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRemoveGroupFromAffinityRule(hcluster : HCLUSTER, rulename : windows_core::PCWSTR, hgroup : HGROUP) -> u32);
    ClusterRemoveGroupFromAffinityRule(hcluster.param().abi(), rulename.param().abi(), hgroup.param().abi())
}
#[inline]
pub unsafe fn ClusterRemoveGroupFromGroupSet<P0>(hgroup: P0) -> u32
where
    P0: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRemoveGroupFromGroupSet(hgroup : HGROUP) -> u32);
    ClusterRemoveGroupFromGroupSet(hgroup.param().abi())
}
#[inline]
pub unsafe fn ClusterRemoveGroupFromGroupSetEx<P0, P1>(hgroup: P0, lpszreason: P1) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterRemoveGroupFromGroupSetEx(hgroup : HGROUP, lpszreason : windows_core::PCWSTR) -> u32);
    ClusterRemoveGroupFromGroupSetEx(hgroup.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn ClusterResourceCloseEnum<P0>(hresenum: P0) -> u32
where
    P0: windows_core::Param<HRESENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceCloseEnum(hresenum : HRESENUM) -> u32);
    ClusterResourceCloseEnum(hresenum.param().abi())
}
#[inline]
pub unsafe fn ClusterResourceCloseEnumEx<P0>(hresourceenumex: P0) -> u32
where
    P0: windows_core::Param<HRESENUMEX>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceCloseEnumEx(hresourceenumex : HRESENUMEX) -> u32);
    ClusterResourceCloseEnumEx(hresourceenumex.param().abi())
}
#[inline]
pub unsafe fn ClusterResourceControl<P0, P1>(hresource: P0, hhostnode: P1, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceControl(hresource : HRESOURCE, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    ClusterResourceControl(hresource.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), cbinbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), cboutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ClusterResourceControlAsUser<P0, P1>(hresource: P0, hhostnode: P1, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceControlAsUser(hresource : HRESOURCE, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    ClusterResourceControlAsUser(hresource.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), cbinbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), cboutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ClusterResourceControlAsUserEx<P0, P1, P2>(hresource: P0, hhostnode: P1, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HNODE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceControlAsUserEx(hresource : HRESOURCE, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    ClusterResourceControlAsUserEx(hresource.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), cbinbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), cboutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())), lpszreason.param().abi())
}
#[inline]
pub unsafe fn ClusterResourceControlEx<P0, P1, P2>(hresource: P0, hhostnode: P1, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HNODE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceControlEx(hresource : HRESOURCE, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    ClusterResourceControlEx(hresource.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), cbinbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), cboutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())), lpszreason.param().abi())
}
#[inline]
pub unsafe fn ClusterResourceEnum<P0>(hresenum: P0, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32
where
    P0: windows_core::Param<HRESENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceEnum(hresenum : HRESENUM, dwindex : u32, lpdwtype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    ClusterResourceEnum(hresenum.param().abi(), dwindex, lpdwtype, core::mem::transmute(lpszname), lpcchname)
}
#[inline]
pub unsafe fn ClusterResourceEnumEx<P0>(hresourceenumex: P0, dwindex: u32, pitem: *mut CLUSTER_RESOURCE_ENUM_ITEM, cbitem: *mut u32) -> u32
where
    P0: windows_core::Param<HRESENUMEX>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceEnumEx(hresourceenumex : HRESENUMEX, dwindex : u32, pitem : *mut CLUSTER_RESOURCE_ENUM_ITEM, cbitem : *mut u32) -> u32);
    ClusterResourceEnumEx(hresourceenumex.param().abi(), dwindex, pitem, cbitem)
}
#[inline]
pub unsafe fn ClusterResourceGetEnumCount<P0>(hresenum: P0) -> u32
where
    P0: windows_core::Param<HRESENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceGetEnumCount(hresenum : HRESENUM) -> u32);
    ClusterResourceGetEnumCount(hresenum.param().abi())
}
#[inline]
pub unsafe fn ClusterResourceGetEnumCountEx<P0>(hresourceenumex: P0) -> u32
where
    P0: windows_core::Param<HRESENUMEX>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceGetEnumCountEx(hresourceenumex : HRESENUMEX) -> u32);
    ClusterResourceGetEnumCountEx(hresourceenumex.param().abi())
}
#[inline]
pub unsafe fn ClusterResourceOpenEnum<P0>(hresource: P0, dwtype: u32) -> HRESENUM
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceOpenEnum(hresource : HRESOURCE, dwtype : u32) -> HRESENUM);
    ClusterResourceOpenEnum(hresource.param().abi(), dwtype)
}
#[inline]
pub unsafe fn ClusterResourceOpenEnumEx<P0, P1, P2>(hcluster: P0, lpszproperties: P1, cbproperties: u32, lpszroproperties: P2, cbroproperties: u32, dwflags: u32) -> HRESENUMEX
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceOpenEnumEx(hcluster : HCLUSTER, lpszproperties : windows_core::PCWSTR, cbproperties : u32, lpszroproperties : windows_core::PCWSTR, cbroproperties : u32, dwflags : u32) -> HRESENUMEX);
    ClusterResourceOpenEnumEx(hcluster.param().abi(), lpszproperties.param().abi(), cbproperties, lpszroproperties.param().abi(), cbroproperties, dwflags)
}
#[inline]
pub unsafe fn ClusterResourceTypeCloseEnum<P0>(hrestypeenum: P0) -> u32
where
    P0: windows_core::Param<HRESTYPEENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceTypeCloseEnum(hrestypeenum : HRESTYPEENUM) -> u32);
    ClusterResourceTypeCloseEnum(hrestypeenum.param().abi())
}
#[inline]
pub unsafe fn ClusterResourceTypeControl<P0, P1, P2>(hcluster: P0, lpszresourcetypename: P1, hhostnode: P2, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceTypeControl(hcluster : HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    ClusterResourceTypeControl(hcluster.param().abi(), lpszresourcetypename.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ClusterResourceTypeControlAsUser<P0, P1, P2>(hcluster: P0, lpszresourcetypename: P1, hhostnode: P2, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceTypeControlAsUser(hcluster : HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    ClusterResourceTypeControlAsUser(hcluster.param().abi(), lpszresourcetypename.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ClusterResourceTypeControlAsUserEx<P0, P1, P2, P3>(hcluster: P0, lpszresourcetypename: P1, hhostnode: P2, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P3) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<HNODE>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceTypeControlAsUserEx(hcluster : HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    ClusterResourceTypeControlAsUserEx(hcluster.param().abi(), lpszresourcetypename.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())), lpszreason.param().abi())
}
#[inline]
pub unsafe fn ClusterResourceTypeControlEx<P0, P1, P2, P3>(hcluster: P0, lpszresourcetypename: P1, hhostnode: P2, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P3) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<HNODE>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceTypeControlEx(hcluster : HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    ClusterResourceTypeControlEx(hcluster.param().abi(), lpszresourcetypename.param().abi(), hhostnode.param().abi(), dwcontrolcode, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, core::mem::transmute(lpbytesreturned.unwrap_or(std::ptr::null_mut())), lpszreason.param().abi())
}
#[inline]
pub unsafe fn ClusterResourceTypeEnum<P0>(hrestypeenum: P0, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32
where
    P0: windows_core::Param<HRESTYPEENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceTypeEnum(hrestypeenum : HRESTYPEENUM, dwindex : u32, lpdwtype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    ClusterResourceTypeEnum(hrestypeenum.param().abi(), dwindex, lpdwtype, core::mem::transmute(lpszname), lpcchname)
}
#[inline]
pub unsafe fn ClusterResourceTypeGetEnumCount<P0>(hrestypeenum: P0) -> u32
where
    P0: windows_core::Param<HRESTYPEENUM>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceTypeGetEnumCount(hrestypeenum : HRESTYPEENUM) -> u32);
    ClusterResourceTypeGetEnumCount(hrestypeenum.param().abi())
}
#[inline]
pub unsafe fn ClusterResourceTypeOpenEnum<P0, P1>(hcluster: P0, lpszresourcetypename: P1, dwtype: u32) -> HRESTYPEENUM
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterResourceTypeOpenEnum(hcluster : HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, dwtype : u32) -> HRESTYPEENUM);
    ClusterResourceTypeOpenEnum(hcluster.param().abi(), lpszresourcetypename.param().abi(), dwtype)
}
#[inline]
pub unsafe fn ClusterSetAccountAccess<P0, P1>(hcluster: P0, szaccountsid: P1, dwaccess: u32, dwcontroltype: u32) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterSetAccountAccess(hcluster : HCLUSTER, szaccountsid : windows_core::PCWSTR, dwaccess : u32, dwcontroltype : u32) -> u32);
    ClusterSetAccountAccess(hcluster.param().abi(), szaccountsid.param().abi(), dwaccess, dwcontroltype)
}
#[inline]
pub unsafe fn ClusterSharedVolumeSetSnapshotState<P0>(guidsnapshotset: windows_core::GUID, lpszvolumename: P0, state: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterSharedVolumeSetSnapshotState(guidsnapshotset : windows_core::GUID, lpszvolumename : windows_core::PCWSTR, state : CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE) -> u32);
    ClusterSharedVolumeSetSnapshotState(core::mem::transmute(guidsnapshotset), lpszvolumename.param().abi(), state)
}
#[inline]
pub unsafe fn ClusterUpgradeFunctionalLevel<P0, P1>(hcluster: P0, perform: P1, pfnprogresscallback: PCLUSTER_UPGRADE_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("clusapi.dll" "system" fn ClusterUpgradeFunctionalLevel(hcluster : HCLUSTER, perform : super::super::Foundation:: BOOL, pfnprogresscallback : PCLUSTER_UPGRADE_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void) -> u32);
    ClusterUpgradeFunctionalLevel(hcluster.param().abi(), perform.param().abi(), pfnprogresscallback, core::mem::transmute(pvcallbackarg.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn CreateCluster(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>) -> HCLUSTER {
    windows_targets::link!("clusapi.dll" "system" fn CreateCluster(pconfig : *const CREATE_CLUSTER_CONFIG, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void) -> HCLUSTER);
    CreateCluster(pconfig, pfnprogresscallback, core::mem::transmute(pvcallbackarg.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn CreateClusterAvailabilitySet<P0, P1>(hcluster: P0, lpavailabilitysetname: P1, pavailabilitysetconfig: *const CLUSTER_AVAILABILITY_SET_CONFIG) -> HGROUPSET
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn CreateClusterAvailabilitySet(hcluster : HCLUSTER, lpavailabilitysetname : windows_core::PCWSTR, pavailabilitysetconfig : *const CLUSTER_AVAILABILITY_SET_CONFIG) -> HGROUPSET);
    CreateClusterAvailabilitySet(hcluster.param().abi(), lpavailabilitysetname.param().abi(), pavailabilitysetconfig)
}
#[inline]
pub unsafe fn CreateClusterGroup<P0, P1>(hcluster: P0, lpszgroupname: P1) -> HGROUP
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn CreateClusterGroup(hcluster : HCLUSTER, lpszgroupname : windows_core::PCWSTR) -> HGROUP);
    CreateClusterGroup(hcluster.param().abi(), lpszgroupname.param().abi())
}
#[inline]
pub unsafe fn CreateClusterGroupEx<P0, P1>(hcluster: P0, lpszgroupname: P1, pgroupinfo: Option<*const CLUSTER_CREATE_GROUP_INFO>) -> HGROUP
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn CreateClusterGroupEx(hcluster : HCLUSTER, lpszgroupname : windows_core::PCWSTR, pgroupinfo : *const CLUSTER_CREATE_GROUP_INFO) -> HGROUP);
    CreateClusterGroupEx(hcluster.param().abi(), lpszgroupname.param().abi(), core::mem::transmute(pgroupinfo.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn CreateClusterGroupSet<P0, P1>(hcluster: P0, groupsetname: P1) -> HGROUPSET
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn CreateClusterGroupSet(hcluster : HCLUSTER, groupsetname : windows_core::PCWSTR) -> HGROUPSET);
    CreateClusterGroupSet(hcluster.param().abi(), groupsetname.param().abi())
}
#[inline]
pub unsafe fn CreateClusterNameAccount<P0>(hcluster: P0, pconfig: *const CREATE_CLUSTER_NAME_ACCOUNT, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("clusapi.dll" "system" fn CreateClusterNameAccount(hcluster : HCLUSTER, pconfig : *const CREATE_CLUSTER_NAME_ACCOUNT, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void) -> u32);
    CreateClusterNameAccount(hcluster.param().abi(), pconfig, pfnprogresscallback, core::mem::transmute(pvcallbackarg.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn CreateClusterNotifyPort<P0, P1>(hchange: P0, hcluster: P1, dwfilter: u32, dwnotifykey: usize) -> HCHANGE
where
    P0: windows_core::Param<HCHANGE>,
    P1: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("clusapi.dll" "system" fn CreateClusterNotifyPort(hchange : HCHANGE, hcluster : HCLUSTER, dwfilter : u32, dwnotifykey : usize) -> HCHANGE);
    CreateClusterNotifyPort(hchange.param().abi(), hcluster.param().abi(), dwfilter, dwnotifykey)
}
#[inline]
pub unsafe fn CreateClusterNotifyPortV2<P0, P1>(hchange: P0, hcluster: P1, filters: *const NOTIFY_FILTER_AND_TYPE, dwfiltercount: u32, dwnotifykey: usize) -> HCHANGE
where
    P0: windows_core::Param<HCHANGE>,
    P1: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("clusapi.dll" "system" fn CreateClusterNotifyPortV2(hchange : HCHANGE, hcluster : HCLUSTER, filters : *const NOTIFY_FILTER_AND_TYPE, dwfiltercount : u32, dwnotifykey : usize) -> HCHANGE);
    CreateClusterNotifyPortV2(hchange.param().abi(), hcluster.param().abi(), filters, dwfiltercount, dwnotifykey)
}
#[inline]
pub unsafe fn CreateClusterResource<P0, P1, P2>(hgroup: P0, lpszresourcename: P1, lpszresourcetype: P2, dwflags: u32) -> HRESOURCE
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn CreateClusterResource(hgroup : HGROUP, lpszresourcename : windows_core::PCWSTR, lpszresourcetype : windows_core::PCWSTR, dwflags : u32) -> HRESOURCE);
    CreateClusterResource(hgroup.param().abi(), lpszresourcename.param().abi(), lpszresourcetype.param().abi(), dwflags)
}
#[inline]
pub unsafe fn CreateClusterResourceEx<P0, P1, P2, P3>(hgroup: P0, lpszresourcename: P1, lpszresourcetype: P2, dwflags: u32, lpszreason: P3) -> HRESOURCE
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn CreateClusterResourceEx(hgroup : HGROUP, lpszresourcename : windows_core::PCWSTR, lpszresourcetype : windows_core::PCWSTR, dwflags : u32, lpszreason : windows_core::PCWSTR) -> HRESOURCE);
    CreateClusterResourceEx(hgroup.param().abi(), lpszresourcename.param().abi(), lpszresourcetype.param().abi(), dwflags, lpszreason.param().abi())
}
#[inline]
pub unsafe fn CreateClusterResourceType<P0, P1, P2, P3>(hcluster: P0, lpszresourcetypename: P1, lpszdisplayname: P2, lpszresourcetypedll: P3, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn CreateClusterResourceType(hcluster : HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, lpszdisplayname : windows_core::PCWSTR, lpszresourcetypedll : windows_core::PCWSTR, dwlooksalivepollinterval : u32, dwisalivepollinterval : u32) -> u32);
    CreateClusterResourceType(hcluster.param().abi(), lpszresourcetypename.param().abi(), lpszdisplayname.param().abi(), lpszresourcetypedll.param().abi(), dwlooksalivepollinterval, dwisalivepollinterval)
}
#[inline]
pub unsafe fn CreateClusterResourceTypeEx<P0, P1, P2, P3, P4>(hcluster: P0, lpszresourcetypename: P1, lpszdisplayname: P2, lpszresourcetypedll: P3, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32, lpszreason: P4) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn CreateClusterResourceTypeEx(hcluster : HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, lpszdisplayname : windows_core::PCWSTR, lpszresourcetypedll : windows_core::PCWSTR, dwlooksalivepollinterval : u32, dwisalivepollinterval : u32, lpszreason : windows_core::PCWSTR) -> u32);
    CreateClusterResourceTypeEx(hcluster.param().abi(), lpszresourcetypename.param().abi(), lpszdisplayname.param().abi(), lpszresourcetypedll.param().abi(), dwlooksalivepollinterval, dwisalivepollinterval, lpszreason.param().abi())
}
#[inline]
pub unsafe fn DeleteClusterGroup<P0>(hgroup: P0) -> u32
where
    P0: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn DeleteClusterGroup(hgroup : HGROUP) -> u32);
    DeleteClusterGroup(hgroup.param().abi())
}
#[inline]
pub unsafe fn DeleteClusterGroupEx<P0, P1>(hgroup: P0, lpszreason: P1) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn DeleteClusterGroupEx(hgroup : HGROUP, lpszreason : windows_core::PCWSTR) -> u32);
    DeleteClusterGroupEx(hgroup.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn DeleteClusterGroupSet<P0>(hgroupset: P0) -> u32
where
    P0: windows_core::Param<HGROUPSET>,
{
    windows_targets::link!("clusapi.dll" "system" fn DeleteClusterGroupSet(hgroupset : HGROUPSET) -> u32);
    DeleteClusterGroupSet(hgroupset.param().abi())
}
#[inline]
pub unsafe fn DeleteClusterGroupSetEx<P0, P1>(hgroupset: P0, lpszreason: P1) -> u32
where
    P0: windows_core::Param<HGROUPSET>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn DeleteClusterGroupSetEx(hgroupset : HGROUPSET, lpszreason : windows_core::PCWSTR) -> u32);
    DeleteClusterGroupSetEx(hgroupset.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn DeleteClusterResource<P0>(hresource: P0) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn DeleteClusterResource(hresource : HRESOURCE) -> u32);
    DeleteClusterResource(hresource.param().abi())
}
#[inline]
pub unsafe fn DeleteClusterResourceEx<P0, P1>(hresource: P0, lpszreason: P1) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn DeleteClusterResourceEx(hresource : HRESOURCE, lpszreason : windows_core::PCWSTR) -> u32);
    DeleteClusterResourceEx(hresource.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn DeleteClusterResourceType<P0, P1>(hcluster: P0, lpszresourcetypename: P1) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn DeleteClusterResourceType(hcluster : HCLUSTER, lpszresourcetypename : windows_core::PCWSTR) -> u32);
    DeleteClusterResourceType(hcluster.param().abi(), lpszresourcetypename.param().abi())
}
#[inline]
pub unsafe fn DeleteClusterResourceTypeEx<P0, P1, P2>(hcluster: P0, lpsztypename: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn DeleteClusterResourceTypeEx(hcluster : HCLUSTER, lpsztypename : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    DeleteClusterResourceTypeEx(hcluster.param().abi(), lpsztypename.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn DestroyCluster<P0, P1>(hcluster: P0, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>, fdeletevirtualcomputerobjects: P1) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("clusapi.dll" "system" fn DestroyCluster(hcluster : HCLUSTER, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void, fdeletevirtualcomputerobjects : super::super::Foundation:: BOOL) -> u32);
    DestroyCluster(hcluster.param().abi(), pfnprogresscallback, core::mem::transmute(pvcallbackarg.unwrap_or(std::ptr::null())), fdeletevirtualcomputerobjects.param().abi())
}
#[inline]
pub unsafe fn DestroyClusterGroup<P0>(hgroup: P0) -> u32
where
    P0: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn DestroyClusterGroup(hgroup : HGROUP) -> u32);
    DestroyClusterGroup(hgroup.param().abi())
}
#[inline]
pub unsafe fn DestroyClusterGroupEx<P0, P1>(hgroup: P0, lpszreason: P1) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn DestroyClusterGroupEx(hgroup : HGROUP, lpszreason : windows_core::PCWSTR) -> u32);
    DestroyClusterGroupEx(hgroup.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn DetermineCNOResTypeFromCluster<P0>(hcluster: P0, pcnorestype: *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("clusapi.dll" "system" fn DetermineCNOResTypeFromCluster(hcluster : HCLUSTER, pcnorestype : *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32);
    DetermineCNOResTypeFromCluster(hcluster.param().abi(), pcnorestype)
}
#[inline]
pub unsafe fn DetermineCNOResTypeFromNodelist(cnodes: u32, ppsznodenames: *const windows_core::PCWSTR, pcnorestype: *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32 {
    windows_targets::link!("clusapi.dll" "system" fn DetermineCNOResTypeFromNodelist(cnodes : u32, ppsznodenames : *const windows_core::PCWSTR, pcnorestype : *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32);
    DetermineCNOResTypeFromNodelist(cnodes, ppsznodenames, pcnorestype)
}
#[inline]
pub unsafe fn DetermineClusterCloudTypeFromCluster<P0>(hcluster: P0, pcloudtype: *mut CLUSTER_CLOUD_TYPE) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("clusapi.dll" "system" fn DetermineClusterCloudTypeFromCluster(hcluster : HCLUSTER, pcloudtype : *mut CLUSTER_CLOUD_TYPE) -> u32);
    DetermineClusterCloudTypeFromCluster(hcluster.param().abi(), pcloudtype)
}
#[inline]
pub unsafe fn DetermineClusterCloudTypeFromNodelist(cnodes: u32, ppsznodenames: *const windows_core::PCWSTR, pcloudtype: *mut CLUSTER_CLOUD_TYPE) -> u32 {
    windows_targets::link!("clusapi.dll" "system" fn DetermineClusterCloudTypeFromNodelist(cnodes : u32, ppsznodenames : *const windows_core::PCWSTR, pcloudtype : *mut CLUSTER_CLOUD_TYPE) -> u32);
    DetermineClusterCloudTypeFromNodelist(cnodes, ppsznodenames, pcloudtype)
}
#[inline]
pub unsafe fn EvictClusterNode<P0>(hnode: P0) -> u32
where
    P0: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn EvictClusterNode(hnode : HNODE) -> u32);
    EvictClusterNode(hnode.param().abi())
}
#[inline]
pub unsafe fn EvictClusterNodeEx<P0>(hnode: P0, dwtimeout: u32, phrcleanupstatus: *mut windows_core::HRESULT) -> u32
where
    P0: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn EvictClusterNodeEx(hnode : HNODE, dwtimeout : u32, phrcleanupstatus : *mut windows_core::HRESULT) -> u32);
    EvictClusterNodeEx(hnode.param().abi(), dwtimeout, phrcleanupstatus)
}
#[inline]
pub unsafe fn EvictClusterNodeEx2<P0, P1>(hnode: P0, dwtimeout: u32, phrcleanupstatus: *mut windows_core::HRESULT, lpszreason: P1) -> u32
where
    P0: windows_core::Param<HNODE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn EvictClusterNodeEx2(hnode : HNODE, dwtimeout : u32, phrcleanupstatus : *mut windows_core::HRESULT, lpszreason : windows_core::PCWSTR) -> u32);
    EvictClusterNodeEx2(hnode.param().abi(), dwtimeout, phrcleanupstatus, lpszreason.param().abi())
}
#[inline]
pub unsafe fn FailClusterResource<P0>(hresource: P0) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn FailClusterResource(hresource : HRESOURCE) -> u32);
    FailClusterResource(hresource.param().abi())
}
#[inline]
pub unsafe fn FailClusterResourceEx<P0, P1>(hresource: P0, lpszreason: P1) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn FailClusterResourceEx(hresource : HRESOURCE, lpszreason : windows_core::PCWSTR) -> u32);
    FailClusterResourceEx(hresource.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn FreeClusterCrypt(pcryptinfo: *const core::ffi::c_void) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn FreeClusterCrypt(pcryptinfo : *const core::ffi::c_void) -> u32);
    FreeClusterCrypt(pcryptinfo)
}
#[inline]
pub unsafe fn FreeClusterHealthFault(clusterhealthfault: *mut CLUSTER_HEALTH_FAULT) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn FreeClusterHealthFault(clusterhealthfault : *mut CLUSTER_HEALTH_FAULT) -> u32);
    FreeClusterHealthFault(clusterhealthfault)
}
#[inline]
pub unsafe fn FreeClusterHealthFaultArray(clusterhealthfaultarray: *mut CLUSTER_HEALTH_FAULT_ARRAY) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn FreeClusterHealthFaultArray(clusterhealthfaultarray : *mut CLUSTER_HEALTH_FAULT_ARRAY) -> u32);
    FreeClusterHealthFaultArray(clusterhealthfaultarray)
}
#[inline]
pub unsafe fn GetClusterFromGroup<P0>(hgroup: P0) -> HCLUSTER
where
    P0: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterFromGroup(hgroup : HGROUP) -> HCLUSTER);
    GetClusterFromGroup(hgroup.param().abi())
}
#[inline]
pub unsafe fn GetClusterFromNetInterface<P0>(hnetinterface: P0) -> HCLUSTER
where
    P0: windows_core::Param<HNETINTERFACE>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterFromNetInterface(hnetinterface : HNETINTERFACE) -> HCLUSTER);
    GetClusterFromNetInterface(hnetinterface.param().abi())
}
#[inline]
pub unsafe fn GetClusterFromNetwork<P0>(hnetwork: P0) -> HCLUSTER
where
    P0: windows_core::Param<HNETWORK>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterFromNetwork(hnetwork : HNETWORK) -> HCLUSTER);
    GetClusterFromNetwork(hnetwork.param().abi())
}
#[inline]
pub unsafe fn GetClusterFromNode<P0>(hnode: P0) -> HCLUSTER
where
    P0: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterFromNode(hnode : HNODE) -> HCLUSTER);
    GetClusterFromNode(hnode.param().abi())
}
#[inline]
pub unsafe fn GetClusterFromResource<P0>(hresource: P0) -> HCLUSTER
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterFromResource(hresource : HRESOURCE) -> HCLUSTER);
    GetClusterFromResource(hresource.param().abi())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterGroupKey<P0>(hgroup: P0, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY>
where
    P0: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterGroupKey(hgroup : HGROUP, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = GetClusterGroupKey(hgroup.param().abi(), samdesired);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetClusterGroupState<P0>(hgroup: P0, lpsznodename: windows_core::PWSTR, lpcchnodename: Option<*mut u32>) -> CLUSTER_GROUP_STATE
where
    P0: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterGroupState(hgroup : HGROUP, lpsznodename : windows_core::PWSTR, lpcchnodename : *mut u32) -> CLUSTER_GROUP_STATE);
    GetClusterGroupState(hgroup.param().abi(), core::mem::transmute(lpsznodename), core::mem::transmute(lpcchnodename.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetClusterInformation<P0>(hcluster: P0, lpszclustername: windows_core::PWSTR, lpcchclustername: *mut u32, lpclusterinfo: Option<*mut CLUSTERVERSIONINFO>) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterInformation(hcluster : HCLUSTER, lpszclustername : windows_core::PWSTR, lpcchclustername : *mut u32, lpclusterinfo : *mut CLUSTERVERSIONINFO) -> u32);
    GetClusterInformation(hcluster.param().abi(), core::mem::transmute(lpszclustername), lpcchclustername, core::mem::transmute(lpclusterinfo.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterKey<P0>(hcluster: P0, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY>
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterKey(hcluster : HCLUSTER, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = GetClusterKey(hcluster.param().abi(), samdesired);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetClusterNetInterface<P0, P1, P2>(hcluster: P0, lpsznodename: P1, lpsznetworkname: P2, lpszinterfacename: windows_core::PWSTR, lpcchinterfacename: *mut u32) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterNetInterface(hcluster : HCLUSTER, lpsznodename : windows_core::PCWSTR, lpsznetworkname : windows_core::PCWSTR, lpszinterfacename : windows_core::PWSTR, lpcchinterfacename : *mut u32) -> u32);
    GetClusterNetInterface(hcluster.param().abi(), lpsznodename.param().abi(), lpsznetworkname.param().abi(), core::mem::transmute(lpszinterfacename), lpcchinterfacename)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterNetInterfaceKey<P0>(hnetinterface: P0, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY>
where
    P0: windows_core::Param<HNETINTERFACE>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterNetInterfaceKey(hnetinterface : HNETINTERFACE, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = GetClusterNetInterfaceKey(hnetinterface.param().abi(), samdesired);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetClusterNetInterfaceState<P0>(hnetinterface: P0) -> CLUSTER_NETINTERFACE_STATE
where
    P0: windows_core::Param<HNETINTERFACE>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterNetInterfaceState(hnetinterface : HNETINTERFACE) -> CLUSTER_NETINTERFACE_STATE);
    GetClusterNetInterfaceState(hnetinterface.param().abi())
}
#[inline]
pub unsafe fn GetClusterNetworkId<P0>(hnetwork: P0, lpsznetworkid: windows_core::PWSTR, lpcchname: *mut u32) -> u32
where
    P0: windows_core::Param<HNETWORK>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterNetworkId(hnetwork : HNETWORK, lpsznetworkid : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    GetClusterNetworkId(hnetwork.param().abi(), core::mem::transmute(lpsznetworkid), lpcchname)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterNetworkKey<P0>(hnetwork: P0, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY>
where
    P0: windows_core::Param<HNETWORK>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterNetworkKey(hnetwork : HNETWORK, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = GetClusterNetworkKey(hnetwork.param().abi(), samdesired);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetClusterNetworkState<P0>(hnetwork: P0) -> CLUSTER_NETWORK_STATE
where
    P0: windows_core::Param<HNETWORK>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterNetworkState(hnetwork : HNETWORK) -> CLUSTER_NETWORK_STATE);
    GetClusterNetworkState(hnetwork.param().abi())
}
#[inline]
pub unsafe fn GetClusterNodeId<P0>(hnode: P0, lpsznodeid: windows_core::PWSTR, lpcchname: *mut u32) -> u32
where
    P0: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterNodeId(hnode : HNODE, lpsznodeid : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    GetClusterNodeId(hnode.param().abi(), core::mem::transmute(lpsznodeid), lpcchname)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterNodeKey<P0>(hnode: P0, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY>
where
    P0: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterNodeKey(hnode : HNODE, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = GetClusterNodeKey(hnode.param().abi(), samdesired);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetClusterNodeState<P0>(hnode: P0) -> CLUSTER_NODE_STATE
where
    P0: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterNodeState(hnode : HNODE) -> CLUSTER_NODE_STATE);
    GetClusterNodeState(hnode.param().abi())
}
#[inline]
pub unsafe fn GetClusterNotify<P0>(hchange: P0, lpdwnotifykey: *mut usize, lpdwfiltertype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32, dwmilliseconds: u32) -> u32
where
    P0: windows_core::Param<HCHANGE>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterNotify(hchange : HCHANGE, lpdwnotifykey : *mut usize, lpdwfiltertype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32, dwmilliseconds : u32) -> u32);
    GetClusterNotify(hchange.param().abi(), lpdwnotifykey, lpdwfiltertype, core::mem::transmute(lpszname), lpcchname, dwmilliseconds)
}
#[inline]
pub unsafe fn GetClusterNotifyV2<P0>(hchange: P0, lpdwnotifykey: *mut usize, pfilterandtype: Option<*mut NOTIFY_FILTER_AND_TYPE>, buffer: Option<*mut u8>, lpbbuffersize: Option<*mut u32>, lpszobjectid: windows_core::PWSTR, lpcchobjectid: Option<*mut u32>, lpszparentid: windows_core::PWSTR, lpcchparentid: Option<*mut u32>, lpszname: windows_core::PWSTR, lpcchname: Option<*mut u32>, lpsztype: windows_core::PWSTR, lpcchtype: Option<*mut u32>, dwmilliseconds: u32) -> u32
where
    P0: windows_core::Param<HCHANGE>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterNotifyV2(hchange : HCHANGE, lpdwnotifykey : *mut usize, pfilterandtype : *mut NOTIFY_FILTER_AND_TYPE, buffer : *mut u8, lpbbuffersize : *mut u32, lpszobjectid : windows_core::PWSTR, lpcchobjectid : *mut u32, lpszparentid : windows_core::PWSTR, lpcchparentid : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32, lpsztype : windows_core::PWSTR, lpcchtype : *mut u32, dwmilliseconds : u32) -> u32);
    GetClusterNotifyV2(
        hchange.param().abi(),
        lpdwnotifykey,
        core::mem::transmute(pfilterandtype.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpbbuffersize.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpszobjectid),
        core::mem::transmute(lpcchobjectid.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpszparentid),
        core::mem::transmute(lpcchparentid.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpszname),
        core::mem::transmute(lpcchname.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpsztype),
        core::mem::transmute(lpcchtype.unwrap_or(std::ptr::null_mut())),
        dwmilliseconds,
    )
}
#[inline]
pub unsafe fn GetClusterQuorumResource<P0>(hcluster: P0, lpszresourcename: windows_core::PWSTR, lpcchresourcename: *mut u32, lpszdevicename: windows_core::PWSTR, lpcchdevicename: *mut u32, lpdwmaxquorumlogsize: *mut u32) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterQuorumResource(hcluster : HCLUSTER, lpszresourcename : windows_core::PWSTR, lpcchresourcename : *mut u32, lpszdevicename : windows_core::PWSTR, lpcchdevicename : *mut u32, lpdwmaxquorumlogsize : *mut u32) -> u32);
    GetClusterQuorumResource(hcluster.param().abi(), core::mem::transmute(lpszresourcename), lpcchresourcename, core::mem::transmute(lpszdevicename), lpcchdevicename, lpdwmaxquorumlogsize)
}
#[inline]
pub unsafe fn GetClusterResourceDependencyExpression<P0>(hresource: P0, lpszdependencyexpression: windows_core::PWSTR, lpcchdependencyexpression: *mut u32) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterResourceDependencyExpression(hresource : HRESOURCE, lpszdependencyexpression : windows_core::PWSTR, lpcchdependencyexpression : *mut u32) -> u32);
    GetClusterResourceDependencyExpression(hresource.param().abi(), core::mem::transmute(lpszdependencyexpression), lpcchdependencyexpression)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterResourceKey<P0>(hresource: P0, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY>
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterResourceKey(hresource : HRESOURCE, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = GetClusterResourceKey(hresource.param().abi(), samdesired);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetClusterResourceNetworkName<P0>(hresource: P0, lpbuffer: windows_core::PWSTR, nsize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterResourceNetworkName(hresource : HRESOURCE, lpbuffer : windows_core::PWSTR, nsize : *mut u32) -> super::super::Foundation:: BOOL);
    GetClusterResourceNetworkName(hresource.param().abi(), core::mem::transmute(lpbuffer), nsize).ok()
}
#[inline]
pub unsafe fn GetClusterResourceState<P0>(hresource: P0, lpsznodename: windows_core::PWSTR, lpcchnodename: Option<*mut u32>, lpszgroupname: windows_core::PWSTR, lpcchgroupname: Option<*mut u32>) -> CLUSTER_RESOURCE_STATE
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterResourceState(hresource : HRESOURCE, lpsznodename : windows_core::PWSTR, lpcchnodename : *mut u32, lpszgroupname : windows_core::PWSTR, lpcchgroupname : *mut u32) -> CLUSTER_RESOURCE_STATE);
    GetClusterResourceState(hresource.param().abi(), core::mem::transmute(lpsznodename), core::mem::transmute(lpcchnodename.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpszgroupname), core::mem::transmute(lpcchgroupname.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterResourceTypeKey<P0, P1>(hcluster: P0, lpsztypename: P1, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY>
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetClusterResourceTypeKey(hcluster : HCLUSTER, lpsztypename : windows_core::PCWSTR, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = GetClusterResourceTypeKey(hcluster.param().abi(), lpsztypename.param().abi(), samdesired);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetNodeCloudTypeDW<P0>(ppsznodename: P0, nodecloudtype: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetNodeCloudTypeDW(ppsznodename : windows_core::PCWSTR, nodecloudtype : *mut u32) -> u32);
    GetNodeCloudTypeDW(ppsznodename.param().abi(), nodecloudtype)
}
#[inline]
pub unsafe fn GetNodeClusterState<P0>(lpsznodename: P0, pdwclusterstate: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetNodeClusterState(lpsznodename : windows_core::PCWSTR, pdwclusterstate : *mut u32) -> u32);
    GetNodeClusterState(lpsznodename.param().abi(), pdwclusterstate)
}
#[inline]
pub unsafe fn GetNotifyEventHandle<P0>(hchange: P0, lphtargetevent: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<HCHANGE>,
{
    windows_targets::link!("clusapi.dll" "system" fn GetNotifyEventHandle(hchange : HCHANGE, lphtargetevent : *mut super::super::Foundation:: HANDLE) -> u32);
    GetNotifyEventHandle(hchange.param().abi(), lphtargetevent)
}
#[inline]
pub unsafe fn InitializeClusterHealthFault(clusterhealthfault: *mut CLUSTER_HEALTH_FAULT) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn InitializeClusterHealthFault(clusterhealthfault : *mut CLUSTER_HEALTH_FAULT) -> u32);
    InitializeClusterHealthFault(clusterhealthfault)
}
#[inline]
pub unsafe fn InitializeClusterHealthFaultArray(clusterhealthfaultarray: *mut CLUSTER_HEALTH_FAULT_ARRAY) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn InitializeClusterHealthFaultArray(clusterhealthfaultarray : *mut CLUSTER_HEALTH_FAULT_ARRAY) -> u32);
    InitializeClusterHealthFaultArray(clusterhealthfaultarray)
}
#[inline]
pub unsafe fn IsFileOnClusterSharedVolume<P0>(lpszpathname: P0, pbfileisonsharedvolume: *mut super::super::Foundation::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn IsFileOnClusterSharedVolume(lpszpathname : windows_core::PCWSTR, pbfileisonsharedvolume : *mut super::super::Foundation:: BOOL) -> u32);
    IsFileOnClusterSharedVolume(lpszpathname.param().abi(), pbfileisonsharedvolume)
}
#[inline]
pub unsafe fn MoveClusterGroup<P0, P1>(hgroup: P0, hdestinationnode: P1) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn MoveClusterGroup(hgroup : HGROUP, hdestinationnode : HNODE) -> u32);
    MoveClusterGroup(hgroup.param().abi(), hdestinationnode.param().abi())
}
#[inline]
pub unsafe fn MoveClusterGroupEx<P0, P1>(hgroup: P0, hdestinationnode: P1, dwmoveflags: u32, lpinbuffer: Option<&[u8]>) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn MoveClusterGroupEx(hgroup : HGROUP, hdestinationnode : HNODE, dwmoveflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32) -> u32);
    MoveClusterGroupEx(hgroup.param().abi(), hdestinationnode.param().abi(), dwmoveflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn MoveClusterGroupEx2<P0, P1, P2>(hgroup: P0, hdestinationnode: P1, dwmoveflags: u32, lpinbuffer: Option<&[u8]>, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HNODE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn MoveClusterGroupEx2(hgroup : HGROUP, hdestinationnode : HNODE, dwmoveflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    MoveClusterGroupEx2(hgroup.param().abi(), hdestinationnode.param().abi(), dwmoveflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpszreason.param().abi())
}
#[inline]
pub unsafe fn OfflineClusterGroup<P0>(hgroup: P0) -> u32
where
    P0: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn OfflineClusterGroup(hgroup : HGROUP) -> u32);
    OfflineClusterGroup(hgroup.param().abi())
}
#[inline]
pub unsafe fn OfflineClusterGroupEx<P0>(hgroup: P0, dwofflineflags: u32, lpinbuffer: Option<&[u8]>) -> u32
where
    P0: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn OfflineClusterGroupEx(hgroup : HGROUP, dwofflineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32) -> u32);
    OfflineClusterGroupEx(hgroup.param().abi(), dwofflineflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn OfflineClusterGroupEx2<P0, P1>(hgroup: P0, dwofflineflags: u32, lpinbuffer: Option<*const u8>, cbinbuffersize: u32, lpszreason: P1) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OfflineClusterGroupEx2(hgroup : HGROUP, dwofflineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    OfflineClusterGroupEx2(hgroup.param().abi(), dwofflineflags, core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), cbinbuffersize, lpszreason.param().abi())
}
#[inline]
pub unsafe fn OfflineClusterResource<P0>(hresource: P0) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn OfflineClusterResource(hresource : HRESOURCE) -> u32);
    OfflineClusterResource(hresource.param().abi())
}
#[inline]
pub unsafe fn OfflineClusterResourceEx<P0>(hresource: P0, dwofflineflags: u32, lpinbuffer: Option<&[u8]>) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn OfflineClusterResourceEx(hresource : HRESOURCE, dwofflineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32) -> u32);
    OfflineClusterResourceEx(hresource.param().abi(), dwofflineflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn OfflineClusterResourceEx2<P0, P1>(hresource: P0, dwofflineflags: u32, lpinbuffer: Option<&[u8]>, lpszreason: P1) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OfflineClusterResourceEx2(hresource : HRESOURCE, dwofflineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    OfflineClusterResourceEx2(hresource.param().abi(), dwofflineflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpszreason.param().abi())
}
#[inline]
pub unsafe fn OnlineClusterGroup<P0, P1>(hgroup: P0, hdestinationnode: P1) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn OnlineClusterGroup(hgroup : HGROUP, hdestinationnode : HNODE) -> u32);
    OnlineClusterGroup(hgroup.param().abi(), hdestinationnode.param().abi())
}
#[inline]
pub unsafe fn OnlineClusterGroupEx<P0, P1>(hgroup: P0, hdestinationnode: P1, dwonlineflags: u32, lpinbuffer: Option<&[u8]>) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn OnlineClusterGroupEx(hgroup : HGROUP, hdestinationnode : HNODE, dwonlineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32) -> u32);
    OnlineClusterGroupEx(hgroup.param().abi(), hdestinationnode.param().abi(), dwonlineflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn OnlineClusterGroupEx2<P0, P1, P2>(hgroup: P0, hdestinationnode: P1, dwonlineflags: u32, lpinbuffer: Option<&[u8]>, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HNODE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OnlineClusterGroupEx2(hgroup : HGROUP, hdestinationnode : HNODE, dwonlineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    OnlineClusterGroupEx2(hgroup.param().abi(), hdestinationnode.param().abi(), dwonlineflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpszreason.param().abi())
}
#[inline]
pub unsafe fn OnlineClusterResource<P0>(hresource: P0) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn OnlineClusterResource(hresource : HRESOURCE) -> u32);
    OnlineClusterResource(hresource.param().abi())
}
#[inline]
pub unsafe fn OnlineClusterResourceEx<P0>(hresource: P0, dwonlineflags: u32, lpinbuffer: Option<&[u8]>) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn OnlineClusterResourceEx(hresource : HRESOURCE, dwonlineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32) -> u32);
    OnlineClusterResourceEx(hresource.param().abi(), dwonlineflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn OnlineClusterResourceEx2<P0, P1>(hresource: P0, dwonlineflags: u32, lpinbuffer: Option<&[u8]>, lpszreason: P1) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OnlineClusterResourceEx2(hresource : HRESOURCE, dwonlineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    OnlineClusterResourceEx2(hresource.param().abi(), dwonlineflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpszreason.param().abi())
}
#[inline]
pub unsafe fn OpenCluster<P0>(lpszclustername: P0) -> HCLUSTER
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OpenCluster(lpszclustername : windows_core::PCWSTR) -> HCLUSTER);
    OpenCluster(lpszclustername.param().abi())
}
#[inline]
pub unsafe fn OpenClusterCryptProvider<P0>(lpszresource: P0, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> HCLUSCRYPTPROVIDER
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn OpenClusterCryptProvider(lpszresource : windows_core::PCWSTR, lpszprovider : *const i8, dwtype : u32, dwflags : u32) -> HCLUSCRYPTPROVIDER);
    OpenClusterCryptProvider(lpszresource.param().abi(), lpszprovider, dwtype, dwflags)
}
#[inline]
pub unsafe fn OpenClusterCryptProviderEx<P0, P1>(lpszresource: P0, lpszkeyname: P1, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> HCLUSCRYPTPROVIDER
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn OpenClusterCryptProviderEx(lpszresource : windows_core::PCWSTR, lpszkeyname : windows_core::PCWSTR, lpszprovider : *const i8, dwtype : u32, dwflags : u32) -> HCLUSCRYPTPROVIDER);
    OpenClusterCryptProviderEx(lpszresource.param().abi(), lpszkeyname.param().abi(), lpszprovider, dwtype, dwflags)
}
#[inline]
pub unsafe fn OpenClusterEx<P0>(lpszclustername: P0, desiredaccess: u32, grantedaccess: Option<*mut u32>) -> HCLUSTER
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OpenClusterEx(lpszclustername : windows_core::PCWSTR, desiredaccess : u32, grantedaccess : *mut u32) -> HCLUSTER);
    OpenClusterEx(lpszclustername.param().abi(), desiredaccess, core::mem::transmute(grantedaccess.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn OpenClusterGroup<P0, P1>(hcluster: P0, lpszgroupname: P1) -> HGROUP
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OpenClusterGroup(hcluster : HCLUSTER, lpszgroupname : windows_core::PCWSTR) -> HGROUP);
    OpenClusterGroup(hcluster.param().abi(), lpszgroupname.param().abi())
}
#[inline]
pub unsafe fn OpenClusterGroupEx<P0, P1>(hcluster: P0, lpszgroupname: P1, dwdesiredaccess: u32, lpdwgrantedaccess: Option<*mut u32>) -> HGROUP
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OpenClusterGroupEx(hcluster : HCLUSTER, lpszgroupname : windows_core::PCWSTR, dwdesiredaccess : u32, lpdwgrantedaccess : *mut u32) -> HGROUP);
    OpenClusterGroupEx(hcluster.param().abi(), lpszgroupname.param().abi(), dwdesiredaccess, core::mem::transmute(lpdwgrantedaccess.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn OpenClusterGroupSet<P0, P1>(hcluster: P0, lpszgroupsetname: P1) -> HGROUPSET
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OpenClusterGroupSet(hcluster : HCLUSTER, lpszgroupsetname : windows_core::PCWSTR) -> HGROUPSET);
    OpenClusterGroupSet(hcluster.param().abi(), lpszgroupsetname.param().abi())
}
#[inline]
pub unsafe fn OpenClusterNetInterface<P0, P1>(hcluster: P0, lpszinterfacename: P1) -> HNETINTERFACE
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OpenClusterNetInterface(hcluster : HCLUSTER, lpszinterfacename : windows_core::PCWSTR) -> HNETINTERFACE);
    OpenClusterNetInterface(hcluster.param().abi(), lpszinterfacename.param().abi())
}
#[inline]
pub unsafe fn OpenClusterNetInterfaceEx<P0, P1>(hcluster: P0, lpszinterfacename: P1, dwdesiredaccess: u32, lpdwgrantedaccess: Option<*mut u32>) -> HNETINTERFACE
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OpenClusterNetInterfaceEx(hcluster : HCLUSTER, lpszinterfacename : windows_core::PCWSTR, dwdesiredaccess : u32, lpdwgrantedaccess : *mut u32) -> HNETINTERFACE);
    OpenClusterNetInterfaceEx(hcluster.param().abi(), lpszinterfacename.param().abi(), dwdesiredaccess, core::mem::transmute(lpdwgrantedaccess.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn OpenClusterNetwork<P0, P1>(hcluster: P0, lpsznetworkname: P1) -> HNETWORK
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OpenClusterNetwork(hcluster : HCLUSTER, lpsznetworkname : windows_core::PCWSTR) -> HNETWORK);
    OpenClusterNetwork(hcluster.param().abi(), lpsznetworkname.param().abi())
}
#[inline]
pub unsafe fn OpenClusterNetworkEx<P0, P1>(hcluster: P0, lpsznetworkname: P1, dwdesiredaccess: u32, lpdwgrantedaccess: Option<*mut u32>) -> HNETWORK
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OpenClusterNetworkEx(hcluster : HCLUSTER, lpsznetworkname : windows_core::PCWSTR, dwdesiredaccess : u32, lpdwgrantedaccess : *mut u32) -> HNETWORK);
    OpenClusterNetworkEx(hcluster.param().abi(), lpsznetworkname.param().abi(), dwdesiredaccess, core::mem::transmute(lpdwgrantedaccess.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn OpenClusterNode<P0, P1>(hcluster: P0, lpsznodename: P1) -> HNODE
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OpenClusterNode(hcluster : HCLUSTER, lpsznodename : windows_core::PCWSTR) -> HNODE);
    OpenClusterNode(hcluster.param().abi(), lpsznodename.param().abi())
}
#[inline]
pub unsafe fn OpenClusterNodeById<P0>(hcluster: P0, nodeid: u32) -> HNODE
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("clusapi.dll" "system" fn OpenClusterNodeById(hcluster : HCLUSTER, nodeid : u32) -> HNODE);
    OpenClusterNodeById(hcluster.param().abi(), nodeid)
}
#[inline]
pub unsafe fn OpenClusterNodeEx<P0, P1>(hcluster: P0, lpsznodename: P1, dwdesiredaccess: u32, lpdwgrantedaccess: Option<*mut u32>) -> HNODE
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OpenClusterNodeEx(hcluster : HCLUSTER, lpsznodename : windows_core::PCWSTR, dwdesiredaccess : u32, lpdwgrantedaccess : *mut u32) -> HNODE);
    OpenClusterNodeEx(hcluster.param().abi(), lpsznodename.param().abi(), dwdesiredaccess, core::mem::transmute(lpdwgrantedaccess.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn OpenClusterResource<P0, P1>(hcluster: P0, lpszresourcename: P1) -> HRESOURCE
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OpenClusterResource(hcluster : HCLUSTER, lpszresourcename : windows_core::PCWSTR) -> HRESOURCE);
    OpenClusterResource(hcluster.param().abi(), lpszresourcename.param().abi())
}
#[inline]
pub unsafe fn OpenClusterResourceEx<P0, P1>(hcluster: P0, lpszresourcename: P1, dwdesiredaccess: u32, lpdwgrantedaccess: Option<*mut u32>) -> HRESOURCE
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn OpenClusterResourceEx(hcluster : HCLUSTER, lpszresourcename : windows_core::PCWSTR, dwdesiredaccess : u32, lpdwgrantedaccess : *mut u32) -> HRESOURCE);
    OpenClusterResourceEx(hcluster.param().abi(), lpszresourcename.param().abi(), dwdesiredaccess, core::mem::transmute(lpdwgrantedaccess.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PauseClusterNode<P0>(hnode: P0) -> u32
where
    P0: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn PauseClusterNode(hnode : HNODE) -> u32);
    PauseClusterNode(hnode.param().abi())
}
#[inline]
pub unsafe fn PauseClusterNodeEx<P0, P1, P2>(hnode: P0, bdrainnode: P1, dwpauseflags: u32, hnodedraintarget: P2) -> u32
where
    P0: windows_core::Param<HNODE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn PauseClusterNodeEx(hnode : HNODE, bdrainnode : super::super::Foundation:: BOOL, dwpauseflags : u32, hnodedraintarget : HNODE) -> u32);
    PauseClusterNodeEx(hnode.param().abi(), bdrainnode.param().abi(), dwpauseflags, hnodedraintarget.param().abi())
}
#[inline]
pub unsafe fn PauseClusterNodeEx2<P0, P1, P2, P3>(hnode: P0, bdrainnode: P1, dwpauseflags: u32, hnodedraintarget: P2, lpszreason: P3) -> u32
where
    P0: windows_core::Param<HNODE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<HNODE>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn PauseClusterNodeEx2(hnode : HNODE, bdrainnode : super::super::Foundation:: BOOL, dwpauseflags : u32, hnodedraintarget : HNODE, lpszreason : windows_core::PCWSTR) -> u32);
    PauseClusterNodeEx2(hnode.param().abi(), bdrainnode.param().abi(), dwpauseflags, hnodedraintarget.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn QueryAppInstanceVersion(appinstanceid: *const windows_core::GUID, instanceversionhigh: *mut u64, instanceversionlow: *mut u64, versionstatus: *mut super::super::Foundation::NTSTATUS) -> u32 {
    windows_targets::link!("ntlanman.dll" "system" fn QueryAppInstanceVersion(appinstanceid : *const windows_core::GUID, instanceversionhigh : *mut u64, instanceversionlow : *mut u64, versionstatus : *mut super::super::Foundation:: NTSTATUS) -> u32);
    QueryAppInstanceVersion(appinstanceid, instanceversionhigh, instanceversionlow, versionstatus)
}
#[inline]
pub unsafe fn RegisterAppInstance<P0, P1>(processhandle: P0, appinstanceid: *const windows_core::GUID, childreninheritappinstance: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("ntlanman.dll" "system" fn RegisterAppInstance(processhandle : super::super::Foundation:: HANDLE, appinstanceid : *const windows_core::GUID, childreninheritappinstance : super::super::Foundation:: BOOL) -> u32);
    RegisterAppInstance(processhandle.param().abi(), appinstanceid, childreninheritappinstance.param().abi())
}
#[inline]
pub unsafe fn RegisterAppInstanceVersion(appinstanceid: *const windows_core::GUID, instanceversionhigh: u64, instanceversionlow: u64) -> u32 {
    windows_targets::link!("ntlanman.dll" "system" fn RegisterAppInstanceVersion(appinstanceid : *const windows_core::GUID, instanceversionhigh : u64, instanceversionlow : u64) -> u32);
    RegisterAppInstanceVersion(appinstanceid, instanceversionhigh, instanceversionlow)
}
#[inline]
pub unsafe fn RegisterClusterNotify<P0, P1>(hchange: P0, dwfiltertype: u32, hobject: P1, dwnotifykey: usize) -> u32
where
    P0: windows_core::Param<HCHANGE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clusapi.dll" "system" fn RegisterClusterNotify(hchange : HCHANGE, dwfiltertype : u32, hobject : super::super::Foundation:: HANDLE, dwnotifykey : usize) -> u32);
    RegisterClusterNotify(hchange.param().abi(), dwfiltertype, hobject.param().abi(), dwnotifykey)
}
#[inline]
pub unsafe fn RegisterClusterNotifyV2<P0, P1>(hchange: P0, filter: NOTIFY_FILTER_AND_TYPE, hobject: P1, dwnotifykey: usize) -> u32
where
    P0: windows_core::Param<HCHANGE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("clusapi.dll" "system" fn RegisterClusterNotifyV2(hchange : HCHANGE, filter : NOTIFY_FILTER_AND_TYPE, hobject : super::super::Foundation:: HANDLE, dwnotifykey : usize) -> u32);
    RegisterClusterNotifyV2(hchange.param().abi(), core::mem::transmute(filter), hobject.param().abi(), dwnotifykey)
}
#[inline]
pub unsafe fn RegisterClusterResourceTypeNotifyV2<P0, P1, P2>(hchange: P0, hcluster: P1, flags: i64, restypename: P2, dwnotifykey: usize) -> u32
where
    P0: windows_core::Param<HCHANGE>,
    P1: windows_core::Param<HCLUSTER>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn RegisterClusterResourceTypeNotifyV2(hchange : HCHANGE, hcluster : HCLUSTER, flags : i64, restypename : windows_core::PCWSTR, dwnotifykey : usize) -> u32);
    RegisterClusterResourceTypeNotifyV2(hchange.param().abi(), hcluster.param().abi(), flags, restypename.param().abi(), dwnotifykey)
}
#[inline]
pub unsafe fn RemoveClusterGroupDependency<P0, P1>(hgroup: P0, hdependson: P1) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn RemoveClusterGroupDependency(hgroup : HGROUP, hdependson : HGROUP) -> u32);
    RemoveClusterGroupDependency(hgroup.param().abi(), hdependson.param().abi())
}
#[inline]
pub unsafe fn RemoveClusterGroupDependencyEx<P0, P1, P2>(hgroup: P0, hdependson: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HGROUP>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn RemoveClusterGroupDependencyEx(hgroup : HGROUP, hdependson : HGROUP, lpszreason : windows_core::PCWSTR) -> u32);
    RemoveClusterGroupDependencyEx(hgroup.param().abi(), hdependson.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn RemoveClusterGroupSetDependency<P0, P1>(hgroupset: P0, hdependson: P1) -> u32
where
    P0: windows_core::Param<HGROUPSET>,
    P1: windows_core::Param<HGROUPSET>,
{
    windows_targets::link!("clusapi.dll" "system" fn RemoveClusterGroupSetDependency(hgroupset : HGROUPSET, hdependson : HGROUPSET) -> u32);
    RemoveClusterGroupSetDependency(hgroupset.param().abi(), hdependson.param().abi())
}
#[inline]
pub unsafe fn RemoveClusterGroupSetDependencyEx<P0, P1, P2>(hgroupset: P0, hdependson: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HGROUPSET>,
    P1: windows_core::Param<HGROUPSET>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn RemoveClusterGroupSetDependencyEx(hgroupset : HGROUPSET, hdependson : HGROUPSET, lpszreason : windows_core::PCWSTR) -> u32);
    RemoveClusterGroupSetDependencyEx(hgroupset.param().abi(), hdependson.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn RemoveClusterGroupToGroupSetDependency<P0, P1>(hgroup: P0, hdependson: P1) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HGROUPSET>,
{
    windows_targets::link!("clusapi.dll" "system" fn RemoveClusterGroupToGroupSetDependency(hgroup : HGROUP, hdependson : HGROUPSET) -> u32);
    RemoveClusterGroupToGroupSetDependency(hgroup.param().abi(), hdependson.param().abi())
}
#[inline]
pub unsafe fn RemoveClusterGroupToGroupSetDependencyEx<P0, P1, P2>(hgroup: P0, hdependson: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HGROUPSET>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn RemoveClusterGroupToGroupSetDependencyEx(hgroup : HGROUP, hdependson : HGROUPSET, lpszreason : windows_core::PCWSTR) -> u32);
    RemoveClusterGroupToGroupSetDependencyEx(hgroup.param().abi(), hdependson.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn RemoveClusterNameAccount<P0, P1>(hcluster: P0, bdeletecomputerobjects: P1) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("clusapi.dll" "system" fn RemoveClusterNameAccount(hcluster : HCLUSTER, bdeletecomputerobjects : super::super::Foundation:: BOOL) -> u32);
    RemoveClusterNameAccount(hcluster.param().abi(), bdeletecomputerobjects.param().abi())
}
#[inline]
pub unsafe fn RemoveClusterResourceDependency<P0, P1>(hresource: P0, hdependson: P1) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn RemoveClusterResourceDependency(hresource : HRESOURCE, hdependson : HRESOURCE) -> u32);
    RemoveClusterResourceDependency(hresource.param().abi(), hdependson.param().abi())
}
#[inline]
pub unsafe fn RemoveClusterResourceDependencyEx<P0, P1, P2>(hresource: P0, hdependson: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HRESOURCE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn RemoveClusterResourceDependencyEx(hresource : HRESOURCE, hdependson : HRESOURCE, lpszreason : windows_core::PCWSTR) -> u32);
    RemoveClusterResourceDependencyEx(hresource.param().abi(), hdependson.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn RemoveClusterResourceNode<P0, P1>(hresource: P0, hnode: P1) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn RemoveClusterResourceNode(hresource : HRESOURCE, hnode : HNODE) -> u32);
    RemoveClusterResourceNode(hresource.param().abi(), hnode.param().abi())
}
#[inline]
pub unsafe fn RemoveClusterResourceNodeEx<P0, P1, P2>(hresource: P0, hnode: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HNODE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn RemoveClusterResourceNodeEx(hresource : HRESOURCE, hnode : HNODE, lpszreason : windows_core::PCWSTR) -> u32);
    RemoveClusterResourceNodeEx(hresource.param().abi(), hnode.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn RemoveClusterStorageNode<P0, P1>(hcluster: P0, lpszclusterstorageenclosurename: P1, dwtimeout: u32, dwflags: u32) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn RemoveClusterStorageNode(hcluster : HCLUSTER, lpszclusterstorageenclosurename : windows_core::PCWSTR, dwtimeout : u32, dwflags : u32) -> u32);
    RemoveClusterStorageNode(hcluster.param().abi(), lpszclusterstorageenclosurename.param().abi(), dwtimeout, dwflags)
}
#[inline]
pub unsafe fn RemoveCrossClusterGroupSetDependency<P0, P1, P2>(hdependentgroupset: P0, lpremoteclustername: P1, lpremotegroupsetname: P2) -> u32
where
    P0: windows_core::Param<HGROUPSET>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn RemoveCrossClusterGroupSetDependency(hdependentgroupset : HGROUPSET, lpremoteclustername : windows_core::PCWSTR, lpremotegroupsetname : windows_core::PCWSTR) -> u32);
    RemoveCrossClusterGroupSetDependency(hdependentgroupset.param().abi(), lpremoteclustername.param().abi(), lpremotegroupsetname.param().abi())
}
#[inline]
pub unsafe fn RemoveResourceFromClusterSharedVolumes<P0>(hresource: P0) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn RemoveResourceFromClusterSharedVolumes(hresource : HRESOURCE) -> u32);
    RemoveResourceFromClusterSharedVolumes(hresource.param().abi())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilAddUnknownProperties<P0>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut core::ffi::c_void, pcboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilAddUnknownProperties(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, poutpropertylist : *mut core::ffi::c_void, pcboutpropertylistsize : u32, pcbbytesreturned : *mut u32, pcbrequired : *mut u32) -> u32);
    ResUtilAddUnknownProperties(hkeyclusterkey.param().abi(), ppropertytable, poutpropertylist, pcboutpropertylistsize, pcbbytesreturned, pcbrequired)
}
#[inline]
pub unsafe fn ResUtilCreateDirectoryTree<P0>(pszpath: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilCreateDirectoryTree(pszpath : windows_core::PCWSTR) -> u32);
    ResUtilCreateDirectoryTree(pszpath.param().abi())
}
#[inline]
pub unsafe fn ResUtilDupGroup<P0>(group: P0, copy: *mut HGROUP) -> u32
where
    P0: windows_core::Param<HGROUP>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilDupGroup(group : HGROUP, copy : *mut HGROUP) -> u32);
    ResUtilDupGroup(group.param().abi(), copy)
}
#[inline]
pub unsafe fn ResUtilDupParameterBlock(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn ResUtilDupParameterBlock(poutparams : *mut u8, pinparams : *const u8, ppropertytable : *const RESUTIL_PROPERTY_ITEM) -> u32);
    ResUtilDupParameterBlock(poutparams, pinparams, ppropertytable)
}
#[inline]
pub unsafe fn ResUtilDupResource<P0>(group: P0, copy: *mut HRESOURCE) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilDupResource(group : HRESOURCE, copy : *mut HRESOURCE) -> u32);
    ResUtilDupResource(group.param().abi(), copy)
}
#[inline]
pub unsafe fn ResUtilDupString<P0>(pszinstring: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilDupString(pszinstring : windows_core::PCWSTR) -> windows_core::PWSTR);
    ResUtilDupString(pszinstring.param().abi())
}
#[inline]
pub unsafe fn ResUtilEnumGroups<P0, P1>(hcluster: P0, hself: P1, prescallback: LPGROUP_CALLBACK_EX, pparameter: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<HGROUP>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilEnumGroups(hcluster : HCLUSTER, hself : HGROUP, prescallback : LPGROUP_CALLBACK_EX, pparameter : *mut core::ffi::c_void) -> u32);
    ResUtilEnumGroups(hcluster.param().abi(), hself.param().abi(), prescallback, pparameter)
}
#[inline]
pub unsafe fn ResUtilEnumGroupsEx<P0, P1>(hcluster: P0, hself: P1, grouptype: CLUSGROUP_TYPE, prescallback: LPGROUP_CALLBACK_EX, pparameter: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<HGROUP>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilEnumGroupsEx(hcluster : HCLUSTER, hself : HGROUP, grouptype : CLUSGROUP_TYPE, prescallback : LPGROUP_CALLBACK_EX, pparameter : *mut core::ffi::c_void) -> u32);
    ResUtilEnumGroupsEx(hcluster.param().abi(), hself.param().abi(), grouptype, prescallback, pparameter)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilEnumPrivateProperties<P0>(hkeyclusterkey: P0, pszoutproperties: windows_core::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilEnumPrivateProperties(hkeyclusterkey : super::super::System::Registry:: HKEY, pszoutproperties : windows_core::PWSTR, cboutpropertiessize : u32, pcbbytesreturned : *mut u32, pcbrequired : *mut u32) -> u32);
    ResUtilEnumPrivateProperties(hkeyclusterkey.param().abi(), core::mem::transmute(pszoutproperties), cboutpropertiessize, pcbbytesreturned, pcbrequired)
}
#[inline]
pub unsafe fn ResUtilEnumProperties(ppropertytable: *const RESUTIL_PROPERTY_ITEM, pszoutproperties: windows_core::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn ResUtilEnumProperties(ppropertytable : *const RESUTIL_PROPERTY_ITEM, pszoutproperties : windows_core::PWSTR, cboutpropertiessize : u32, pcbbytesreturned : *mut u32, pcbrequired : *mut u32) -> u32);
    ResUtilEnumProperties(ppropertytable, core::mem::transmute(pszoutproperties), cboutpropertiessize, pcbbytesreturned, pcbrequired)
}
#[inline]
pub unsafe fn ResUtilEnumResources<P0, P1>(hself: P0, lpszrestypename: P1, prescallback: LPRESOURCE_CALLBACK, pparameter: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilEnumResources(hself : HRESOURCE, lpszrestypename : windows_core::PCWSTR, prescallback : LPRESOURCE_CALLBACK, pparameter : *mut core::ffi::c_void) -> u32);
    ResUtilEnumResources(hself.param().abi(), lpszrestypename.param().abi(), prescallback, pparameter)
}
#[inline]
pub unsafe fn ResUtilEnumResourcesEx<P0, P1, P2>(hcluster: P0, hself: P1, lpszrestypename: P2, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<HRESOURCE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilEnumResourcesEx(hcluster : HCLUSTER, hself : HRESOURCE, lpszrestypename : windows_core::PCWSTR, prescallback : LPRESOURCE_CALLBACK_EX, pparameter : *mut core::ffi::c_void) -> u32);
    ResUtilEnumResourcesEx(hcluster.param().abi(), hself.param().abi(), lpszrestypename.param().abi(), prescallback, pparameter)
}
#[inline]
pub unsafe fn ResUtilEnumResourcesEx2<P0, P1, P2>(hcluster: P0, hself: P1, lpszrestypename: P2, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut core::ffi::c_void, dwdesiredaccess: u32) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<HRESOURCE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilEnumResourcesEx2(hcluster : HCLUSTER, hself : HRESOURCE, lpszrestypename : windows_core::PCWSTR, prescallback : LPRESOURCE_CALLBACK_EX, pparameter : *mut core::ffi::c_void, dwdesiredaccess : u32) -> u32);
    ResUtilEnumResourcesEx2(hcluster.param().abi(), hself.param().abi(), lpszrestypename.param().abi(), prescallback, pparameter, dwdesiredaccess)
}
#[inline]
pub unsafe fn ResUtilExpandEnvironmentStrings<P0>(pszsrc: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilExpandEnvironmentStrings(pszsrc : windows_core::PCWSTR) -> windows_core::PWSTR);
    ResUtilExpandEnvironmentStrings(pszsrc.param().abi())
}
#[inline]
pub unsafe fn ResUtilFindBinaryProperty<P0>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, pbpropertyvalue: Option<*mut *mut u8>, pcbpropertyvaluesize: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilFindBinaryProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, pbpropertyvalue : *mut *mut u8, pcbpropertyvaluesize : *mut u32) -> u32);
    ResUtilFindBinaryProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), core::mem::transmute(pbpropertyvalue.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcbpropertyvaluesize.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ResUtilFindDependentDiskResourceDriveLetter<P0, P1>(hcluster: P0, hresource: P1, pszdriveletter: windows_core::PWSTR, pcchdriveletter: *mut u32) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilFindDependentDiskResourceDriveLetter(hcluster : HCLUSTER, hresource : HRESOURCE, pszdriveletter : windows_core::PWSTR, pcchdriveletter : *mut u32) -> u32);
    ResUtilFindDependentDiskResourceDriveLetter(hcluster.param().abi(), hresource.param().abi(), core::mem::transmute(pszdriveletter), pcchdriveletter)
}
#[inline]
pub unsafe fn ResUtilFindDwordProperty<P0>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, pdwpropertyvalue: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilFindDwordProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, pdwpropertyvalue : *mut u32) -> u32);
    ResUtilFindDwordProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), pdwpropertyvalue)
}
#[inline]
pub unsafe fn ResUtilFindExpandSzProperty<P0>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, pszpropertyvalue: Option<*mut windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilFindExpandSzProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, pszpropertyvalue : *mut windows_core::PWSTR) -> u32);
    ResUtilFindExpandSzProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), core::mem::transmute(pszpropertyvalue.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ResUtilFindExpandedSzProperty<P0>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, pszpropertyvalue: Option<*mut windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilFindExpandedSzProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, pszpropertyvalue : *mut windows_core::PWSTR) -> u32);
    ResUtilFindExpandedSzProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), core::mem::transmute(pszpropertyvalue.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ResUtilFindFileTimeProperty<P0>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, pftpropertyvalue: *mut super::super::Foundation::FILETIME) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilFindFileTimeProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, pftpropertyvalue : *mut super::super::Foundation:: FILETIME) -> u32);
    ResUtilFindFileTimeProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), pftpropertyvalue)
}
#[inline]
pub unsafe fn ResUtilFindLongProperty<P0>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, plpropertyvalue: *mut i32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilFindLongProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, plpropertyvalue : *mut i32) -> u32);
    ResUtilFindLongProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), plpropertyvalue)
}
#[inline]
pub unsafe fn ResUtilFindMultiSzProperty<P0>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, pszpropertyvalue: *mut windows_core::PWSTR, pcbpropertyvaluesize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilFindMultiSzProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, pszpropertyvalue : *mut windows_core::PWSTR, pcbpropertyvaluesize : *mut u32) -> u32);
    ResUtilFindMultiSzProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), pszpropertyvalue, pcbpropertyvaluesize)
}
#[inline]
pub unsafe fn ResUtilFindSzProperty<P0>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, pszpropertyvalue: Option<*mut windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilFindSzProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, pszpropertyvalue : *mut windows_core::PWSTR) -> u32);
    ResUtilFindSzProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), core::mem::transmute(pszpropertyvalue.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ResUtilFindULargeIntegerProperty<P0>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P0, plpropertyvalue: *mut u64) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilFindULargeIntegerProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, plpropertyvalue : *mut u64) -> u32);
    ResUtilFindULargeIntegerProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), plpropertyvalue)
}
#[inline]
pub unsafe fn ResUtilFreeEnvironment(lpenvironment: *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn ResUtilFreeEnvironment(lpenvironment : *mut core::ffi::c_void) -> u32);
    ResUtilFreeEnvironment(lpenvironment)
}
#[inline]
pub unsafe fn ResUtilFreeParameterBlock(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM) {
    windows_targets::link!("resutils.dll" "system" fn ResUtilFreeParameterBlock(poutparams : *mut u8, pinparams : *const u8, ppropertytable : *const RESUTIL_PROPERTY_ITEM));
    ResUtilFreeParameterBlock(poutparams, pinparams, ppropertytable)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetAllProperties<P0>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetAllProperties(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, poutpropertylist : *mut core::ffi::c_void, cboutpropertylistsize : u32, pcbbytesreturned : *mut u32, pcbrequired : *mut u32) -> u32);
    ResUtilGetAllProperties(hkeyclusterkey.param().abi(), ppropertytable, poutpropertylist, cboutpropertylistsize, pcbbytesreturned, pcbrequired)
}
#[inline]
pub unsafe fn ResUtilGetBinaryProperty(ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_BINARY, pboldvalue: Option<&[u8]>, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetBinaryProperty(ppboutvalue : *mut *mut u8, pcboutvaluesize : *mut u32, pvaluestruct : *const CLUSPROP_BINARY, pboldvalue : *const u8, cboldvaluesize : u32, pppropertylist : *mut *mut u8, pcbpropertylistsize : *mut u32) -> u32);
    ResUtilGetBinaryProperty(ppboutvalue, pcboutvaluesize, pvaluestruct, core::mem::transmute(pboldvalue.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboldvalue.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pppropertylist, pcbpropertylistsize)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetBinaryValue<P0, P1>(hkeyclusterkey: P0, pszvaluename: P1, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetBinaryValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, ppboutvalue : *mut *mut u8, pcboutvaluesize : *mut u32) -> u32);
    ResUtilGetBinaryValue(hkeyclusterkey.param().abi(), pszvaluename.param().abi(), ppboutvalue, pcboutvaluesize)
}
#[inline]
pub unsafe fn ResUtilGetClusterGroupType<P0>(hgroup: P0, grouptype: *mut CLUSGROUP_TYPE) -> u32
where
    P0: windows_core::Param<HGROUP>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetClusterGroupType(hgroup : HGROUP, grouptype : *mut CLUSGROUP_TYPE) -> u32);
    ResUtilGetClusterGroupType(hgroup.param().abi(), grouptype)
}
#[inline]
pub unsafe fn ResUtilGetClusterId<P0>(hcluster: P0, guid: *mut windows_core::GUID) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetClusterId(hcluster : HCLUSTER, guid : *mut windows_core::GUID) -> u32);
    ResUtilGetClusterId(hcluster.param().abi(), guid)
}
#[inline]
pub unsafe fn ResUtilGetClusterRoleState<P0>(hcluster: P0, eclusterrole: CLUSTER_ROLE) -> CLUSTER_ROLE_STATE
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetClusterRoleState(hcluster : HCLUSTER, eclusterrole : CLUSTER_ROLE) -> CLUSTER_ROLE_STATE);
    ResUtilGetClusterRoleState(hcluster.param().abi(), eclusterrole)
}
#[inline]
pub unsafe fn ResUtilGetCoreClusterResources<P0>(hcluster: P0, phclusternameresource: *mut HRESOURCE, phclusteripaddressresource: *mut HRESOURCE, phclusterquorumresource: *mut HRESOURCE) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetCoreClusterResources(hcluster : HCLUSTER, phclusternameresource : *mut HRESOURCE, phclusteripaddressresource : *mut HRESOURCE, phclusterquorumresource : *mut HRESOURCE) -> u32);
    ResUtilGetCoreClusterResources(hcluster.param().abi(), phclusternameresource, phclusteripaddressresource, phclusterquorumresource)
}
#[inline]
pub unsafe fn ResUtilGetCoreClusterResourcesEx<P0>(hclusterin: P0, phclusternameresourceout: Option<*mut HRESOURCE>, phclusterquorumresourceout: Option<*mut HRESOURCE>, dwdesiredaccess: u32) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetCoreClusterResourcesEx(hclusterin : HCLUSTER, phclusternameresourceout : *mut HRESOURCE, phclusterquorumresourceout : *mut HRESOURCE, dwdesiredaccess : u32) -> u32);
    ResUtilGetCoreClusterResourcesEx(hclusterin.param().abi(), core::mem::transmute(phclusternameresourceout.unwrap_or(std::ptr::null_mut())), core::mem::transmute(phclusterquorumresourceout.unwrap_or(std::ptr::null_mut())), dwdesiredaccess)
}
#[inline]
pub unsafe fn ResUtilGetCoreGroup<P0>(hcluster: P0) -> HGROUP
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetCoreGroup(hcluster : HCLUSTER) -> HGROUP);
    ResUtilGetCoreGroup(hcluster.param().abi())
}
#[inline]
pub unsafe fn ResUtilGetDwordProperty(pdwoutvalue: *mut u32, pvaluestruct: *const CLUSPROP_DWORD, dwoldvalue: u32, dwminimum: u32, dwmaximum: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetDwordProperty(pdwoutvalue : *mut u32, pvaluestruct : *const CLUSPROP_DWORD, dwoldvalue : u32, dwminimum : u32, dwmaximum : u32, pppropertylist : *mut *mut u8, pcbpropertylistsize : *mut u32) -> u32);
    ResUtilGetDwordProperty(pdwoutvalue, pvaluestruct, dwoldvalue, dwminimum, dwmaximum, pppropertylist, pcbpropertylistsize)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetDwordValue<P0, P1>(hkeyclusterkey: P0, pszvaluename: P1, pdwoutvalue: *mut u32, dwdefaultvalue: u32) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetDwordValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, pdwoutvalue : *mut u32, dwdefaultvalue : u32) -> u32);
    ResUtilGetDwordValue(hkeyclusterkey.param().abi(), pszvaluename.param().abi(), pdwoutvalue, dwdefaultvalue)
}
#[inline]
pub unsafe fn ResUtilGetEnvironmentWithNetName<P0>(hresource: P0) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetEnvironmentWithNetName(hresource : HRESOURCE) -> *mut core::ffi::c_void);
    ResUtilGetEnvironmentWithNetName(hresource.param().abi())
}
#[inline]
pub unsafe fn ResUtilGetFileTimeProperty(pftoutvalue: *mut super::super::Foundation::FILETIME, pvaluestruct: *const CLUSPROP_FILETIME, ftoldvalue: super::super::Foundation::FILETIME, ftminimum: super::super::Foundation::FILETIME, ftmaximum: super::super::Foundation::FILETIME, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetFileTimeProperty(pftoutvalue : *mut super::super::Foundation:: FILETIME, pvaluestruct : *const CLUSPROP_FILETIME, ftoldvalue : super::super::Foundation:: FILETIME, ftminimum : super::super::Foundation:: FILETIME, ftmaximum : super::super::Foundation:: FILETIME, pppropertylist : *mut *mut u8, pcbpropertylistsize : *mut u32) -> u32);
    ResUtilGetFileTimeProperty(pftoutvalue, pvaluestruct, core::mem::transmute(ftoldvalue), core::mem::transmute(ftminimum), core::mem::transmute(ftmaximum), pppropertylist, pcbpropertylistsize)
}
#[inline]
pub unsafe fn ResUtilGetLongProperty(ploutvalue: *mut i32, pvaluestruct: *const CLUSPROP_LONG, loldvalue: i32, lminimum: i32, lmaximum: i32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetLongProperty(ploutvalue : *mut i32, pvaluestruct : *const CLUSPROP_LONG, loldvalue : i32, lminimum : i32, lmaximum : i32, pppropertylist : *mut *mut u8, pcbpropertylistsize : *mut u32) -> u32);
    ResUtilGetLongProperty(ploutvalue, pvaluestruct, loldvalue, lminimum, lmaximum, pppropertylist, pcbpropertylistsize)
}
#[inline]
pub unsafe fn ResUtilGetMultiSzProperty<P0>(ppszoutvalue: *mut windows_core::PWSTR, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: P0, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetMultiSzProperty(ppszoutvalue : *mut windows_core::PWSTR, pcboutvaluesize : *mut u32, pvaluestruct : *const CLUSPROP_SZ, pszoldvalue : windows_core::PCWSTR, cboldvaluesize : u32, pppropertylist : *mut *mut u8, pcbpropertylistsize : *mut u32) -> u32);
    ResUtilGetMultiSzProperty(ppszoutvalue, pcboutvaluesize, pvaluestruct, pszoldvalue.param().abi(), cboldvaluesize, pppropertylist, pcbpropertylistsize)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetPrivateProperties<P0>(hkeyclusterkey: P0, poutpropertylist: *mut core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetPrivateProperties(hkeyclusterkey : super::super::System::Registry:: HKEY, poutpropertylist : *mut core::ffi::c_void, cboutpropertylistsize : u32, pcbbytesreturned : *mut u32, pcbrequired : *mut u32) -> u32);
    ResUtilGetPrivateProperties(hkeyclusterkey.param().abi(), poutpropertylist, cboutpropertylistsize, pcbbytesreturned, pcbrequired)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetProperties<P0>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetProperties(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, poutpropertylist : *mut core::ffi::c_void, cboutpropertylistsize : u32, pcbbytesreturned : *mut u32, pcbrequired : *mut u32) -> u32);
    ResUtilGetProperties(hkeyclusterkey.param().abi(), ppropertytable, poutpropertylist, cboutpropertylistsize, pcbbytesreturned, pcbrequired)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetPropertiesToParameterBlock<P0, P1>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutparams: *mut u8, bcheckforrequiredproperties: P1, psznameofpropinerror: *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetPropertiesToParameterBlock(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, poutparams : *mut u8, bcheckforrequiredproperties : super::super::Foundation:: BOOL, psznameofpropinerror : *mut windows_core::PWSTR) -> u32);
    ResUtilGetPropertiesToParameterBlock(hkeyclusterkey.param().abi(), ppropertytable, poutparams, bcheckforrequiredproperties.param().abi(), psznameofpropinerror)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetProperty<P0>(hkeyclusterkey: P0, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, poutpropertyitem: *mut *mut core::ffi::c_void, pcboutpropertyitemsize: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetProperty(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytableitem : *const RESUTIL_PROPERTY_ITEM, poutpropertyitem : *mut *mut core::ffi::c_void, pcboutpropertyitemsize : *mut u32) -> u32);
    ResUtilGetProperty(hkeyclusterkey.param().abi(), ppropertytableitem, poutpropertyitem, pcboutpropertyitemsize)
}
#[inline]
pub unsafe fn ResUtilGetPropertyFormats(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertyformatlist: *mut core::ffi::c_void, cbpropertyformatlistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetPropertyFormats(ppropertytable : *const RESUTIL_PROPERTY_ITEM, poutpropertyformatlist : *mut core::ffi::c_void, cbpropertyformatlistsize : u32, pcbbytesreturned : *mut u32, pcbrequired : *mut u32) -> u32);
    ResUtilGetPropertyFormats(ppropertytable, poutpropertyformatlist, cbpropertyformatlistsize, pcbbytesreturned, pcbrequired)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetPropertySize<P0>(hkeyclusterkey: P0, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, pcboutpropertylistsize: *mut u32, pnpropertycount: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetPropertySize(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytableitem : *const RESUTIL_PROPERTY_ITEM, pcboutpropertylistsize : *mut u32, pnpropertycount : *mut u32) -> u32);
    ResUtilGetPropertySize(hkeyclusterkey.param().abi(), ppropertytableitem, pcboutpropertylistsize, pnpropertycount)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetQwordValue<P0, P1>(hkeyclusterkey: P0, pszvaluename: P1, pqwoutvalue: *mut u64, qwdefaultvalue: u64) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetQwordValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, pqwoutvalue : *mut u64, qwdefaultvalue : u64) -> u32);
    ResUtilGetQwordValue(hkeyclusterkey.param().abi(), pszvaluename.param().abi(), pqwoutvalue, qwdefaultvalue)
}
#[inline]
pub unsafe fn ResUtilGetResourceDependency<P0, P1>(hself: P0, lpszresourcetype: P1) -> HRESOURCE
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetResourceDependency(hself : super::super::Foundation:: HANDLE, lpszresourcetype : windows_core::PCWSTR) -> HRESOURCE);
    ResUtilGetResourceDependency(hself.param().abi(), lpszresourcetype.param().abi())
}
#[inline]
pub unsafe fn ResUtilGetResourceDependencyByClass<P0, P1, P2>(hcluster: P0, hself: P1, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: P2) -> HRESOURCE
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetResourceDependencyByClass(hcluster : HCLUSTER, hself : super::super::Foundation:: HANDLE, prci : *mut CLUS_RESOURCE_CLASS_INFO, brecurse : super::super::Foundation:: BOOL) -> HRESOURCE);
    ResUtilGetResourceDependencyByClass(hcluster.param().abi(), hself.param().abi(), prci, brecurse.param().abi())
}
#[inline]
pub unsafe fn ResUtilGetResourceDependencyByClassEx<P0, P1, P2>(hcluster: P0, hself: P1, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: P2, dwdesiredaccess: u32) -> HRESOURCE
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetResourceDependencyByClassEx(hcluster : HCLUSTER, hself : super::super::Foundation:: HANDLE, prci : *mut CLUS_RESOURCE_CLASS_INFO, brecurse : super::super::Foundation:: BOOL, dwdesiredaccess : u32) -> HRESOURCE);
    ResUtilGetResourceDependencyByClassEx(hcluster.param().abi(), hself.param().abi(), prci, brecurse.param().abi(), dwdesiredaccess)
}
#[inline]
pub unsafe fn ResUtilGetResourceDependencyByName<P0, P1, P2, P3>(hcluster: P0, hself: P1, lpszresourcetype: P2, brecurse: P3) -> HRESOURCE
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetResourceDependencyByName(hcluster : HCLUSTER, hself : super::super::Foundation:: HANDLE, lpszresourcetype : windows_core::PCWSTR, brecurse : super::super::Foundation:: BOOL) -> HRESOURCE);
    ResUtilGetResourceDependencyByName(hcluster.param().abi(), hself.param().abi(), lpszresourcetype.param().abi(), brecurse.param().abi())
}
#[inline]
pub unsafe fn ResUtilGetResourceDependencyByNameEx<P0, P1, P2, P3>(hcluster: P0, hself: P1, lpszresourcetype: P2, brecurse: P3, dwdesiredaccess: u32) -> HRESOURCE
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetResourceDependencyByNameEx(hcluster : HCLUSTER, hself : super::super::Foundation:: HANDLE, lpszresourcetype : windows_core::PCWSTR, brecurse : super::super::Foundation:: BOOL, dwdesiredaccess : u32) -> HRESOURCE);
    ResUtilGetResourceDependencyByNameEx(hcluster.param().abi(), hself.param().abi(), lpszresourcetype.param().abi(), brecurse.param().abi(), dwdesiredaccess)
}
#[inline]
pub unsafe fn ResUtilGetResourceDependencyEx<P0, P1>(hself: P0, lpszresourcetype: P1, dwdesiredaccess: u32) -> HRESOURCE
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetResourceDependencyEx(hself : super::super::Foundation:: HANDLE, lpszresourcetype : windows_core::PCWSTR, dwdesiredaccess : u32) -> HRESOURCE);
    ResUtilGetResourceDependencyEx(hself.param().abi(), lpszresourcetype.param().abi(), dwdesiredaccess)
}
#[inline]
pub unsafe fn ResUtilGetResourceDependentIPAddressProps<P0>(hresource: P0, pszaddress: windows_core::PWSTR, pcchaddress: *mut u32, pszsubnetmask: windows_core::PWSTR, pcchsubnetmask: *mut u32, psznetwork: windows_core::PWSTR, pcchnetwork: *mut u32) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetResourceDependentIPAddressProps(hresource : HRESOURCE, pszaddress : windows_core::PWSTR, pcchaddress : *mut u32, pszsubnetmask : windows_core::PWSTR, pcchsubnetmask : *mut u32, psznetwork : windows_core::PWSTR, pcchnetwork : *mut u32) -> u32);
    ResUtilGetResourceDependentIPAddressProps(hresource.param().abi(), core::mem::transmute(pszaddress), pcchaddress, core::mem::transmute(pszsubnetmask), pcchsubnetmask, core::mem::transmute(psznetwork), pcchnetwork)
}
#[inline]
pub unsafe fn ResUtilGetResourceName<P0>(hresource: P0, pszresourcename: windows_core::PWSTR, pcchresourcenameinout: *mut u32) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetResourceName(hresource : HRESOURCE, pszresourcename : windows_core::PWSTR, pcchresourcenameinout : *mut u32) -> u32);
    ResUtilGetResourceName(hresource.param().abi(), core::mem::transmute(pszresourcename), pcchresourcenameinout)
}
#[inline]
pub unsafe fn ResUtilGetResourceNameDependency<P0, P1>(lpszresourcename: P0, lpszresourcetype: P1) -> HRESOURCE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetResourceNameDependency(lpszresourcename : windows_core::PCWSTR, lpszresourcetype : windows_core::PCWSTR) -> HRESOURCE);
    ResUtilGetResourceNameDependency(lpszresourcename.param().abi(), lpszresourcetype.param().abi())
}
#[inline]
pub unsafe fn ResUtilGetResourceNameDependencyEx<P0, P1>(lpszresourcename: P0, lpszresourcetype: P1, dwdesiredaccess: u32) -> HRESOURCE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetResourceNameDependencyEx(lpszresourcename : windows_core::PCWSTR, lpszresourcetype : windows_core::PCWSTR, dwdesiredaccess : u32) -> HRESOURCE);
    ResUtilGetResourceNameDependencyEx(lpszresourcename.param().abi(), lpszresourcetype.param().abi(), dwdesiredaccess)
}
#[inline]
pub unsafe fn ResUtilGetSzProperty<P0>(ppszoutvalue: *mut windows_core::PWSTR, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: P0, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetSzProperty(ppszoutvalue : *mut windows_core::PWSTR, pvaluestruct : *const CLUSPROP_SZ, pszoldvalue : windows_core::PCWSTR, pppropertylist : *mut *mut u8, pcbpropertylistsize : *mut u32) -> u32);
    ResUtilGetSzProperty(ppszoutvalue, pvaluestruct, pszoldvalue.param().abi(), pppropertylist, pcbpropertylistsize)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetSzValue<P0, P1>(hkeyclusterkey: P0, pszvaluename: P1) -> windows_core::PWSTR
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGetSzValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR) -> windows_core::PWSTR);
    ResUtilGetSzValue(hkeyclusterkey.param().abi(), pszvaluename.param().abi())
}
#[inline]
pub unsafe fn ResUtilGroupsEqual<P0, P1>(hself: P0, hgroup: P1, pequal: *mut super::super::Foundation::BOOL) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<HGROUP>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilGroupsEqual(hself : HGROUP, hgroup : HGROUP, pequal : *mut super::super::Foundation:: BOOL) -> u32);
    ResUtilGroupsEqual(hself.param().abi(), hgroup.param().abi(), pequal)
}
#[inline]
pub unsafe fn ResUtilIsPathValid<P0>(pszpath: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilIsPathValid(pszpath : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    ResUtilIsPathValid(pszpath.param().abi())
}
#[inline]
pub unsafe fn ResUtilIsResourceClassEqual<P0>(prci: *mut CLUS_RESOURCE_CLASS_INFO, hresource: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilIsResourceClassEqual(prci : *mut CLUS_RESOURCE_CLASS_INFO, hresource : HRESOURCE) -> super::super::Foundation:: BOOL);
    ResUtilIsResourceClassEqual(prci, hresource.param().abi())
}
#[inline]
pub unsafe fn ResUtilLeftPaxosIsLessThanRight(left: *const PaxosTagCStruct, right: *const PaxosTagCStruct) -> super::super::Foundation::BOOL {
    windows_targets::link!("resutils.dll" "system" fn ResUtilLeftPaxosIsLessThanRight(left : *const PaxosTagCStruct, right : *const PaxosTagCStruct) -> super::super::Foundation:: BOOL);
    ResUtilLeftPaxosIsLessThanRight(left, right)
}
#[inline]
pub unsafe fn ResUtilNodeEnum<P0>(hcluster: P0, pnodecallback: LPNODE_CALLBACK, pparameter: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilNodeEnum(hcluster : HCLUSTER, pnodecallback : LPNODE_CALLBACK, pparameter : *mut core::ffi::c_void) -> u32);
    ResUtilNodeEnum(hcluster.param().abi(), pnodecallback, pparameter)
}
#[inline]
pub unsafe fn ResUtilPaxosComparer(left: *const PaxosTagCStruct, right: *const PaxosTagCStruct) -> super::super::Foundation::BOOL {
    windows_targets::link!("resutils.dll" "system" fn ResUtilPaxosComparer(left : *const PaxosTagCStruct, right : *const PaxosTagCStruct) -> super::super::Foundation:: BOOL);
    ResUtilPaxosComparer(left, right)
}
#[inline]
pub unsafe fn ResUtilPropertyListFromParameterBlock(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: Option<*mut core::ffi::c_void>, pcboutpropertylistsize: *mut u32, pinparams: *const u8, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn ResUtilPropertyListFromParameterBlock(ppropertytable : *const RESUTIL_PROPERTY_ITEM, poutpropertylist : *mut core::ffi::c_void, pcboutpropertylistsize : *mut u32, pinparams : *const u8, pcbbytesreturned : *mut u32, pcbrequired : *mut u32) -> u32);
    ResUtilPropertyListFromParameterBlock(ppropertytable, core::mem::transmute(poutpropertylist.unwrap_or(std::ptr::null_mut())), pcboutpropertylistsize, pinparams, pcbbytesreturned, pcbrequired)
}
#[inline]
pub unsafe fn ResUtilRemoveResourceServiceEnvironment<P0>(pszservicename: P0, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilRemoveResourceServiceEnvironment(pszservicename : windows_core::PCWSTR, pfnlogevent : PLOG_EVENT_ROUTINE, hresourcehandle : isize) -> u32);
    ResUtilRemoveResourceServiceEnvironment(pszservicename.param().abi(), pfnlogevent, hresourcehandle)
}
#[inline]
pub unsafe fn ResUtilResourceDepEnum<P0>(hself: P0, enumtype: u32, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilResourceDepEnum(hself : HRESOURCE, enumtype : u32, prescallback : LPRESOURCE_CALLBACK_EX, pparameter : *mut core::ffi::c_void) -> u32);
    ResUtilResourceDepEnum(hself.param().abi(), enumtype, prescallback, pparameter)
}
#[inline]
pub unsafe fn ResUtilResourceTypesEqual<P0, P1>(lpszresourcetypename: P0, hresource: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilResourceTypesEqual(lpszresourcetypename : windows_core::PCWSTR, hresource : HRESOURCE) -> super::super::Foundation:: BOOL);
    ResUtilResourceTypesEqual(lpszresourcetypename.param().abi(), hresource.param().abi())
}
#[inline]
pub unsafe fn ResUtilResourcesEqual<P0, P1>(hself: P0, hresource: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilResourcesEqual(hself : HRESOURCE, hresource : HRESOURCE) -> super::super::Foundation:: BOOL);
    ResUtilResourcesEqual(hself.param().abi(), hresource.param().abi())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetBinaryValue<P0, P1>(hkeyclusterkey: P0, pszvaluename: P1, pbnewvalue: &[u8], ppboutvalue: Option<*mut *mut u8>, pcboutvaluesize: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilSetBinaryValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, pbnewvalue : *const u8, cbnewvaluesize : u32, ppboutvalue : *mut *mut u8, pcboutvaluesize : *mut u32) -> u32);
    ResUtilSetBinaryValue(hkeyclusterkey.param().abi(), pszvaluename.param().abi(), core::mem::transmute(pbnewvalue.as_ptr()), pbnewvalue.len().try_into().unwrap(), core::mem::transmute(ppboutvalue.unwrap_or(std::ptr::null_mut())), pcboutvaluesize)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetDwordValue<P0, P1>(hkeyclusterkey: P0, pszvaluename: P1, dwnewvalue: u32, pdwoutvalue: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilSetDwordValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, dwnewvalue : u32, pdwoutvalue : *mut u32) -> u32);
    ResUtilSetDwordValue(hkeyclusterkey.param().abi(), pszvaluename.param().abi(), dwnewvalue, pdwoutvalue)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetExpandSzValue<P0, P1, P2>(hkeyclusterkey: P0, pszvaluename: P1, psznewvalue: P2, ppszoutstring: Option<*mut windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilSetExpandSzValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, psznewvalue : windows_core::PCWSTR, ppszoutstring : *mut windows_core::PWSTR) -> u32);
    ResUtilSetExpandSzValue(hkeyclusterkey.param().abi(), pszvaluename.param().abi(), psznewvalue.param().abi(), core::mem::transmute(ppszoutstring.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetMultiSzValue<P0, P1, P2>(hkeyclusterkey: P0, pszvaluename: P1, psznewvalue: P2, cbnewvaluesize: u32, ppszoutvalue: Option<*mut windows_core::PWSTR>, pcboutvaluesize: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilSetMultiSzValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, psznewvalue : windows_core::PCWSTR, cbnewvaluesize : u32, ppszoutvalue : *mut windows_core::PWSTR, pcboutvaluesize : *mut u32) -> u32);
    ResUtilSetMultiSzValue(hkeyclusterkey.param().abi(), pszvaluename.param().abi(), psznewvalue.param().abi(), cbnewvaluesize, core::mem::transmute(ppszoutvalue.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcboutvaluesize.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetPrivatePropertyList<P0>(hkeyclusterkey: P0, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilSetPrivatePropertyList(hkeyclusterkey : super::super::System::Registry:: HKEY, pinpropertylist : *const core::ffi::c_void, cbinpropertylistsize : u32) -> u32);
    ResUtilSetPrivatePropertyList(hkeyclusterkey.param().abi(), pinpropertylist, cbinpropertylistsize)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetPropertyParameterBlock<P0>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilSetPropertyParameterBlock(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, reserved : *mut core::ffi::c_void, pinparams : *const u8, pinpropertylist : *const core::ffi::c_void, cbinpropertylistsize : u32, poutparams : *mut u8) -> u32);
    ResUtilSetPropertyParameterBlock(hkeyclusterkey.param().abi(), ppropertytable, reserved, pinparams, pinpropertylist, cbinpropertylistsize, poutparams)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetPropertyParameterBlockEx<P0, P1>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: P1, poutparams: *mut u8) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilSetPropertyParameterBlockEx(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, reserved : *mut core::ffi::c_void, pinparams : *const u8, pinpropertylist : *const core::ffi::c_void, cbinpropertylistsize : u32, bforcewrite : super::super::Foundation:: BOOL, poutparams : *mut u8) -> u32);
    ResUtilSetPropertyParameterBlockEx(hkeyclusterkey.param().abi(), ppropertytable, reserved, pinparams, pinpropertylist, cbinpropertylistsize, bforcewrite.param().abi(), poutparams)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetPropertyTable<P0, P1>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: Option<*const core::ffi::c_void>, ballowunknownproperties: P1, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, poutparams: Option<*mut u8>) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilSetPropertyTable(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, reserved : *const core::ffi::c_void, ballowunknownproperties : super::super::Foundation:: BOOL, pinpropertylist : *const core::ffi::c_void, cbinpropertylistsize : u32, poutparams : *mut u8) -> u32);
    ResUtilSetPropertyTable(hkeyclusterkey.param().abi(), ppropertytable, core::mem::transmute(reserved.unwrap_or(std::ptr::null())), ballowunknownproperties.param().abi(), pinpropertylist, cbinpropertylistsize, core::mem::transmute(poutparams.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetPropertyTableEx<P0, P1, P2>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut core::ffi::c_void, ballowunknownproperties: P1, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: P2, poutparams: *mut u8) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilSetPropertyTableEx(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, reserved : *mut core::ffi::c_void, ballowunknownproperties : super::super::Foundation:: BOOL, pinpropertylist : *const core::ffi::c_void, cbinpropertylistsize : u32, bforcewrite : super::super::Foundation:: BOOL, poutparams : *mut u8) -> u32);
    ResUtilSetPropertyTableEx(hkeyclusterkey.param().abi(), ppropertytable, reserved, ballowunknownproperties.param().abi(), pinpropertylist, cbinpropertylistsize, bforcewrite.param().abi(), poutparams)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetQwordValue<P0, P1>(hkeyclusterkey: P0, pszvaluename: P1, qwnewvalue: u64, pqwoutvalue: Option<*mut u64>) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilSetQwordValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, qwnewvalue : u64, pqwoutvalue : *mut u64) -> u32);
    ResUtilSetQwordValue(hkeyclusterkey.param().abi(), pszvaluename.param().abi(), qwnewvalue, core::mem::transmute(pqwoutvalue.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ResUtilSetResourceServiceEnvironment<P0, P1>(pszservicename: P0, hresource: P1, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilSetResourceServiceEnvironment(pszservicename : windows_core::PCWSTR, hresource : HRESOURCE, pfnlogevent : PLOG_EVENT_ROUTINE, hresourcehandle : isize) -> u32);
    ResUtilSetResourceServiceEnvironment(pszservicename.param().abi(), hresource.param().abi(), pfnlogevent, hresourcehandle)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ResUtilSetResourceServiceStartParameters<P0, P1>(pszservicename: P0, schscmhandle: P1, phservice: *mut super::super::Security::SC_HANDLE, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilSetResourceServiceStartParameters(pszservicename : windows_core::PCWSTR, schscmhandle : super::super::Security:: SC_HANDLE, phservice : *mut super::super::Security:: SC_HANDLE, pfnlogevent : PLOG_EVENT_ROUTINE, hresourcehandle : isize) -> u32);
    ResUtilSetResourceServiceStartParameters(pszservicename.param().abi(), schscmhandle.param().abi(), phservice, pfnlogevent, hresourcehandle)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ResUtilSetResourceServiceStartParametersEx<P0, P1>(pszservicename: P0, schscmhandle: P1, phservice: *mut super::super::Security::SC_HANDLE, dwdesiredaccess: u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilSetResourceServiceStartParametersEx(pszservicename : windows_core::PCWSTR, schscmhandle : super::super::Security:: SC_HANDLE, phservice : *mut super::super::Security:: SC_HANDLE, dwdesiredaccess : u32, pfnlogevent : PLOG_EVENT_ROUTINE, hresourcehandle : isize) -> u32);
    ResUtilSetResourceServiceStartParametersEx(pszservicename.param().abi(), schscmhandle.param().abi(), phservice, dwdesiredaccess, pfnlogevent, hresourcehandle)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetSzValue<P0, P1, P2>(hkeyclusterkey: P0, pszvaluename: P1, psznewvalue: P2, ppszoutstring: Option<*mut windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilSetSzValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, psznewvalue : windows_core::PCWSTR, ppszoutstring : *mut windows_core::PWSTR) -> u32);
    ResUtilSetSzValue(hkeyclusterkey.param().abi(), pszvaluename.param().abi(), psznewvalue.param().abi(), core::mem::transmute(ppszoutstring.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetUnknownProperties<P0>(hkeyclusterkey: P0, ppropertytable: *const RESUTIL_PROPERTY_ITEM, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilSetUnknownProperties(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, pinpropertylist : *const core::ffi::c_void, cbinpropertylistsize : u32) -> u32);
    ResUtilSetUnknownProperties(hkeyclusterkey.param().abi(), ppropertytable, pinpropertylist, cbinpropertylistsize)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetValueEx<P0, P1>(hkeyclusterkey: P0, valuename: P1, valuetype: u32, valuedata: &[u8], flags: u32) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilSetValueEx(hkeyclusterkey : super::super::System::Registry:: HKEY, valuename : windows_core::PCWSTR, valuetype : u32, valuedata : *const u8, valuesize : u32, flags : u32) -> u32);
    ResUtilSetValueEx(hkeyclusterkey.param().abi(), valuename.param().abi(), valuetype, core::mem::transmute(valuedata.as_ptr()), valuedata.len().try_into().unwrap(), flags)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ResUtilStartResourceService<P0>(pszservicename: P0, phservicehandle: *mut super::super::Security::SC_HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilStartResourceService(pszservicename : windows_core::PCWSTR, phservicehandle : *mut super::super::Security:: SC_HANDLE) -> u32);
    ResUtilStartResourceService(pszservicename.param().abi(), phservicehandle)
}
#[inline]
pub unsafe fn ResUtilStopResourceService<P0>(pszservicename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilStopResourceService(pszservicename : windows_core::PCWSTR) -> u32);
    ResUtilStopResourceService(pszservicename.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ResUtilStopService<P0>(hservicehandle: P0) -> u32
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilStopService(hservicehandle : super::super::Security:: SC_HANDLE) -> u32);
    ResUtilStopService(hservicehandle.param().abi())
}
#[inline]
pub unsafe fn ResUtilTerminateServiceProcessFromResDll<P0>(dwservicepid: u32, boffline: P0, pdwresourcestate: *mut u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilTerminateServiceProcessFromResDll(dwservicepid : u32, boffline : super::super::Foundation:: BOOL, pdwresourcestate : *mut u32, pfnlogevent : PLOG_EVENT_ROUTINE, hresourcehandle : isize) -> u32);
    ResUtilTerminateServiceProcessFromResDll(dwservicepid, boffline.param().abi(), pdwresourcestate, pfnlogevent, hresourcehandle)
}
#[inline]
pub unsafe fn ResUtilVerifyPrivatePropertyList(pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn ResUtilVerifyPrivatePropertyList(pinpropertylist : *const core::ffi::c_void, cbinpropertylistsize : u32) -> u32);
    ResUtilVerifyPrivatePropertyList(pinpropertylist, cbinpropertylistsize)
}
#[inline]
pub unsafe fn ResUtilVerifyPropertyTable<P0>(ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: Option<*const core::ffi::c_void>, ballowunknownproperties: P0, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, poutparams: Option<*mut u8>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilVerifyPropertyTable(ppropertytable : *const RESUTIL_PROPERTY_ITEM, reserved : *const core::ffi::c_void, ballowunknownproperties : super::super::Foundation:: BOOL, pinpropertylist : *const core::ffi::c_void, cbinpropertylistsize : u32, poutparams : *mut u8) -> u32);
    ResUtilVerifyPropertyTable(ppropertytable, core::mem::transmute(reserved.unwrap_or(std::ptr::null())), ballowunknownproperties.param().abi(), pinpropertylist, cbinpropertylistsize, core::mem::transmute(poutparams.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ResUtilVerifyResourceService<P0>(pszservicename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilVerifyResourceService(pszservicename : windows_core::PCWSTR) -> u32);
    ResUtilVerifyResourceService(pszservicename.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ResUtilVerifyService<P0>(hservicehandle: P0) -> u32
where
    P0: windows_core::Param<super::super::Security::SC_HANDLE>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilVerifyService(hservicehandle : super::super::Security:: SC_HANDLE) -> u32);
    ResUtilVerifyService(hservicehandle.param().abi())
}
#[inline]
pub unsafe fn ResUtilVerifyShutdownSafe(flags: u32, reason: u32, presult: *mut u32) -> u32 {
    windows_targets::link!("resutils.dll" "system" fn ResUtilVerifyShutdownSafe(flags : u32, reason : u32, presult : *mut u32) -> u32);
    ResUtilVerifyShutdownSafe(flags, reason, presult)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilsDeleteKeyTree<P0, P1, P2>(key: P0, keyname: P1, treatnokeyaserror: P2) -> u32
where
    P0: windows_core::Param<super::super::System::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("resutils.dll" "system" fn ResUtilsDeleteKeyTree(key : super::super::System::Registry:: HKEY, keyname : windows_core::PCWSTR, treatnokeyaserror : super::super::Foundation:: BOOL) -> u32);
    ResUtilsDeleteKeyTree(key.param().abi(), keyname.param().abi(), treatnokeyaserror.param().abi())
}
#[inline]
pub unsafe fn ResetAllAppInstanceVersions() -> u32 {
    windows_targets::link!("ntlanman.dll" "system" fn ResetAllAppInstanceVersions() -> u32);
    ResetAllAppInstanceVersions()
}
#[inline]
pub unsafe fn RestartClusterResource<P0>(hresource: P0, dwflags: u32) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
{
    windows_targets::link!("clusapi.dll" "system" fn RestartClusterResource(hresource : HRESOURCE, dwflags : u32) -> u32);
    RestartClusterResource(hresource.param().abi(), dwflags)
}
#[inline]
pub unsafe fn RestartClusterResourceEx<P0, P1>(hresource: P0, dwflags: u32, lpszreason: P1) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn RestartClusterResourceEx(hresource : HRESOURCE, dwflags : u32, lpszreason : windows_core::PCWSTR) -> u32);
    RestartClusterResourceEx(hresource.param().abi(), dwflags, lpszreason.param().abi())
}
#[inline]
pub unsafe fn RestoreClusterDatabase<P0, P1, P2>(lpszpathname: P0, bforce: P1, lpszquorumdriveletter: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn RestoreClusterDatabase(lpszpathname : windows_core::PCWSTR, bforce : super::super::Foundation:: BOOL, lpszquorumdriveletter : windows_core::PCWSTR) -> u32);
    RestoreClusterDatabase(lpszpathname.param().abi(), bforce.param().abi(), lpszquorumdriveletter.param().abi())
}
#[inline]
pub unsafe fn ResumeClusterNode<P0>(hnode: P0) -> u32
where
    P0: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn ResumeClusterNode(hnode : HNODE) -> u32);
    ResumeClusterNode(hnode.param().abi())
}
#[inline]
pub unsafe fn ResumeClusterNodeEx<P0>(hnode: P0, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32) -> u32
where
    P0: windows_core::Param<HNODE>,
{
    windows_targets::link!("clusapi.dll" "system" fn ResumeClusterNodeEx(hnode : HNODE, eresumefailbacktype : CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved : u32) -> u32);
    ResumeClusterNodeEx(hnode.param().abi(), eresumefailbacktype, dwresumeflagsreserved)
}
#[inline]
pub unsafe fn ResumeClusterNodeEx2<P0, P1>(hnode: P0, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32, lpszreason: P1) -> u32
where
    P0: windows_core::Param<HNODE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn ResumeClusterNodeEx2(hnode : HNODE, eresumefailbacktype : CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved : u32, lpszreason : windows_core::PCWSTR) -> u32);
    ResumeClusterNodeEx2(hnode.param().abi(), eresumefailbacktype, dwresumeflagsreserved, lpszreason.param().abi())
}
#[inline]
pub unsafe fn SetAppInstanceCsvFlags<P0>(processhandle: P0, mask: u32, flags: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ntlanman.dll" "system" fn SetAppInstanceCsvFlags(processhandle : super::super::Foundation:: HANDLE, mask : u32, flags : u32) -> u32);
    SetAppInstanceCsvFlags(processhandle.param().abi(), mask, flags)
}
#[inline]
pub unsafe fn SetClusterGroupName<P0, P1>(hgroup: P0, lpszgroupname: P1) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterGroupName(hgroup : HGROUP, lpszgroupname : windows_core::PCWSTR) -> u32);
    SetClusterGroupName(hgroup.param().abi(), lpszgroupname.param().abi())
}
#[inline]
pub unsafe fn SetClusterGroupNameEx<P0, P1, P2>(hgroup: P0, lpszgroupname: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterGroupNameEx(hgroup : HGROUP, lpszgroupname : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    SetClusterGroupNameEx(hgroup.param().abi(), lpszgroupname.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn SetClusterGroupNodeList<P0>(hgroup: P0, nodelist: Option<&[HNODE]>) -> u32
where
    P0: windows_core::Param<HGROUP>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterGroupNodeList(hgroup : HGROUP, nodecount : u32, nodelist : *const HNODE) -> u32);
    SetClusterGroupNodeList(hgroup.param().abi(), nodelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(nodelist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[inline]
pub unsafe fn SetClusterGroupNodeListEx<P0, P1>(hgroup: P0, nodelist: &[HNODE], lpszreason: P1) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterGroupNodeListEx(hgroup : HGROUP, nodecount : u32, nodelist : *const HNODE, lpszreason : windows_core::PCWSTR) -> u32);
    SetClusterGroupNodeListEx(hgroup.param().abi(), nodelist.len().try_into().unwrap(), core::mem::transmute(nodelist.as_ptr()), lpszreason.param().abi())
}
#[inline]
pub unsafe fn SetClusterGroupSetDependencyExpression<P0, P1>(hgroupset: P0, lpszdependencyexprssion: P1) -> u32
where
    P0: windows_core::Param<HGROUPSET>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterGroupSetDependencyExpression(hgroupset : HGROUPSET, lpszdependencyexprssion : windows_core::PCWSTR) -> u32);
    SetClusterGroupSetDependencyExpression(hgroupset.param().abi(), lpszdependencyexprssion.param().abi())
}
#[inline]
pub unsafe fn SetClusterGroupSetDependencyExpressionEx<P0, P1, P2>(hgroupset: P0, lpszdependencyexpression: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HGROUPSET>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterGroupSetDependencyExpressionEx(hgroupset : HGROUPSET, lpszdependencyexpression : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    SetClusterGroupSetDependencyExpressionEx(hgroupset.param().abi(), lpszdependencyexpression.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn SetClusterName<P0, P1>(hcluster: P0, lpsznewclustername: P1) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterName(hcluster : HCLUSTER, lpsznewclustername : windows_core::PCWSTR) -> u32);
    SetClusterName(hcluster.param().abi(), lpsznewclustername.param().abi())
}
#[inline]
pub unsafe fn SetClusterNameEx<P0, P1, P2>(hcluster: P0, lpsznewclustername: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterNameEx(hcluster : HCLUSTER, lpsznewclustername : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    SetClusterNameEx(hcluster.param().abi(), lpsznewclustername.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn SetClusterNetworkName<P0, P1>(hnetwork: P0, lpszname: P1) -> u32
where
    P0: windows_core::Param<HNETWORK>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterNetworkName(hnetwork : HNETWORK, lpszname : windows_core::PCWSTR) -> u32);
    SetClusterNetworkName(hnetwork.param().abi(), lpszname.param().abi())
}
#[inline]
pub unsafe fn SetClusterNetworkNameEx<P0, P1, P2>(hnetwork: P0, lpszname: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HNETWORK>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterNetworkNameEx(hnetwork : HNETWORK, lpszname : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    SetClusterNetworkNameEx(hnetwork.param().abi(), lpszname.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn SetClusterNetworkPriorityOrder<P0>(hcluster: P0, networklist: &[HNETWORK]) -> u32
where
    P0: windows_core::Param<HCLUSTER>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterNetworkPriorityOrder(hcluster : HCLUSTER, networkcount : u32, networklist : *const HNETWORK) -> u32);
    SetClusterNetworkPriorityOrder(hcluster.param().abi(), networklist.len().try_into().unwrap(), core::mem::transmute(networklist.as_ptr()))
}
#[inline]
pub unsafe fn SetClusterQuorumResource<P0, P1>(hresource: P0, lpszdevicename: P1, dwmaxquologsize: u32) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterQuorumResource(hresource : HRESOURCE, lpszdevicename : windows_core::PCWSTR, dwmaxquologsize : u32) -> u32);
    SetClusterQuorumResource(hresource.param().abi(), lpszdevicename.param().abi(), dwmaxquologsize)
}
#[inline]
pub unsafe fn SetClusterQuorumResourceEx<P0, P1, P2>(hresource: P0, lpszdevicename: P1, dwmaxquorumlogsize: u32, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterQuorumResourceEx(hresource : HRESOURCE, lpszdevicename : windows_core::PCWSTR, dwmaxquorumlogsize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    SetClusterQuorumResourceEx(hresource.param().abi(), lpszdevicename.param().abi(), dwmaxquorumlogsize, lpszreason.param().abi())
}
#[inline]
pub unsafe fn SetClusterResourceDependencyExpression<P0, P1>(hresource: P0, lpszdependencyexpression: P1) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterResourceDependencyExpression(hresource : HRESOURCE, lpszdependencyexpression : windows_core::PCWSTR) -> u32);
    SetClusterResourceDependencyExpression(hresource.param().abi(), lpszdependencyexpression.param().abi())
}
#[inline]
pub unsafe fn SetClusterResourceName<P0, P1>(hresource: P0, lpszresourcename: P1) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterResourceName(hresource : HRESOURCE, lpszresourcename : windows_core::PCWSTR) -> u32);
    SetClusterResourceName(hresource.param().abi(), lpszresourcename.param().abi())
}
#[inline]
pub unsafe fn SetClusterResourceNameEx<P0, P1, P2>(hresource: P0, lpszresourcename: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HRESOURCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterResourceNameEx(hresource : HRESOURCE, lpszresourcename : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    SetClusterResourceNameEx(hresource.param().abi(), lpszresourcename.param().abi(), lpszreason.param().abi())
}
#[inline]
pub unsafe fn SetClusterServiceAccountPassword<P0, P1>(lpszclustername: P0, lpsznewpassword: P1, dwflags: u32, lpreturnstatusbuffer: Option<*mut CLUSTER_SET_PASSWORD_STATUS>, lpcbreturnstatusbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetClusterServiceAccountPassword(lpszclustername : windows_core::PCWSTR, lpsznewpassword : windows_core::PCWSTR, dwflags : u32, lpreturnstatusbuffer : *mut CLUSTER_SET_PASSWORD_STATUS, lpcbreturnstatusbuffersize : *mut u32) -> u32);
    SetClusterServiceAccountPassword(lpszclustername.param().abi(), lpsznewpassword.param().abi(), dwflags, core::mem::transmute(lpreturnstatusbuffer.unwrap_or(std::ptr::null_mut())), lpcbreturnstatusbuffersize)
}
#[inline]
pub unsafe fn SetGroupDependencyExpression<P0, P1>(hgroup: P0, lpszdependencyexpression: P1) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetGroupDependencyExpression(hgroup : HGROUP, lpszdependencyexpression : windows_core::PCWSTR) -> u32);
    SetGroupDependencyExpression(hgroup.param().abi(), lpszdependencyexpression.param().abi())
}
#[inline]
pub unsafe fn SetGroupDependencyExpressionEx<P0, P1, P2>(hgroup: P0, lpszdependencyexpression: P1, lpszreason: P2) -> u32
where
    P0: windows_core::Param<HGROUP>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("clusapi.dll" "system" fn SetGroupDependencyExpressionEx(hgroup : HGROUP, lpszdependencyexpression : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    SetGroupDependencyExpressionEx(hgroup.param().abi(), lpszdependencyexpression.param().abi(), lpszreason.param().abi())
}
windows_core::imp::define_interface!(IGetClusterDataInfo, IGetClusterDataInfo_Vtbl, 0x97dede51_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IGetClusterDataInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetClusterDataInfo, windows_core::IUnknown);
impl IGetClusterDataInfo {
    pub unsafe fn GetClusterName(&self, lpszname: &windows_core::BSTR, pcchname: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClusterName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lpszname), pcchname).ok()
    }
    pub unsafe fn GetClusterHandle(&self) -> HCLUSTER {
        (windows_core::Interface::vtable(self).GetClusterHandle)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetObjectCount(&self) -> i32 {
        (windows_core::Interface::vtable(self).GetObjectCount)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IGetClusterDataInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClusterName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub GetClusterHandle: unsafe extern "system" fn(*mut core::ffi::c_void) -> HCLUSTER,
    pub GetObjectCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
}
windows_core::imp::define_interface!(IGetClusterGroupInfo, IGetClusterGroupInfo_Vtbl, 0x97dede54_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IGetClusterGroupInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetClusterGroupInfo, windows_core::IUnknown);
impl IGetClusterGroupInfo {
    pub unsafe fn GetGroupHandle(&self, lobjindex: i32) -> HGROUP {
        (windows_core::Interface::vtable(self).GetGroupHandle)(windows_core::Interface::as_raw(self), lobjindex)
    }
}
#[repr(C)]
pub struct IGetClusterGroupInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGroupHandle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> HGROUP,
}
windows_core::imp::define_interface!(IGetClusterNetInterfaceInfo, IGetClusterNetInterfaceInfo_Vtbl, 0x97dede57_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IGetClusterNetInterfaceInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetClusterNetInterfaceInfo, windows_core::IUnknown);
impl IGetClusterNetInterfaceInfo {
    pub unsafe fn GetNetInterfaceHandle(&self, lobjindex: i32) -> HNETINTERFACE {
        (windows_core::Interface::vtable(self).GetNetInterfaceHandle)(windows_core::Interface::as_raw(self), lobjindex)
    }
}
#[repr(C)]
pub struct IGetClusterNetInterfaceInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNetInterfaceHandle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> HNETINTERFACE,
}
windows_core::imp::define_interface!(IGetClusterNetworkInfo, IGetClusterNetworkInfo_Vtbl, 0x97dede56_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IGetClusterNetworkInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetClusterNetworkInfo, windows_core::IUnknown);
impl IGetClusterNetworkInfo {
    pub unsafe fn GetNetworkHandle(&self, lobjindex: i32) -> HNETWORK {
        (windows_core::Interface::vtable(self).GetNetworkHandle)(windows_core::Interface::as_raw(self), lobjindex)
    }
}
#[repr(C)]
pub struct IGetClusterNetworkInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNetworkHandle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> HNETWORK,
}
windows_core::imp::define_interface!(IGetClusterNodeInfo, IGetClusterNodeInfo_Vtbl, 0x97dede53_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IGetClusterNodeInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetClusterNodeInfo, windows_core::IUnknown);
impl IGetClusterNodeInfo {
    pub unsafe fn GetNodeHandle(&self, lobjindex: i32) -> HNODE {
        (windows_core::Interface::vtable(self).GetNodeHandle)(windows_core::Interface::as_raw(self), lobjindex)
    }
}
#[repr(C)]
pub struct IGetClusterNodeInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNodeHandle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> HNODE,
}
windows_core::imp::define_interface!(IGetClusterObjectInfo, IGetClusterObjectInfo_Vtbl, 0x97dede52_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IGetClusterObjectInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetClusterObjectInfo, windows_core::IUnknown);
impl IGetClusterObjectInfo {
    pub unsafe fn GetObjectName(&self, lobjindex: i32, lpszname: &windows_core::BSTR, pcchname: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObjectName)(windows_core::Interface::as_raw(self), lobjindex, core::mem::transmute_copy(lpszname), pcchname).ok()
    }
    pub unsafe fn GetObjectType(&self, lobjindex: i32) -> CLUADMEX_OBJECT_TYPE {
        (windows_core::Interface::vtable(self).GetObjectType)(windows_core::Interface::as_raw(self), lobjindex)
    }
}
#[repr(C)]
pub struct IGetClusterObjectInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub GetObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> CLUADMEX_OBJECT_TYPE,
}
windows_core::imp::define_interface!(IGetClusterResourceInfo, IGetClusterResourceInfo_Vtbl, 0x97dede55_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IGetClusterResourceInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetClusterResourceInfo, windows_core::IUnknown);
impl IGetClusterResourceInfo {
    pub unsafe fn GetResourceHandle(&self, lobjindex: i32) -> HRESOURCE {
        (windows_core::Interface::vtable(self).GetResourceHandle)(windows_core::Interface::as_raw(self), lobjindex)
    }
    pub unsafe fn GetResourceTypeName(&self, lobjindex: i32, lpszrestypename: &windows_core::BSTR, pcchrestypename: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetResourceTypeName)(windows_core::Interface::as_raw(self), lobjindex, core::mem::transmute_copy(lpszrestypename), pcchrestypename).ok()
    }
    pub unsafe fn GetResourceNetworkName(&self, lobjindex: i32, lpsznetname: &windows_core::BSTR, pcchnetname: *mut u32) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).GetResourceNetworkName)(windows_core::Interface::as_raw(self), lobjindex, core::mem::transmute_copy(lpsznetname), pcchnetname)
    }
}
#[repr(C)]
pub struct IGetClusterResourceInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetResourceHandle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> HRESOURCE,
    pub GetResourceTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub GetResourceNetworkName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, *mut u32) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(IGetClusterUIInfo, IGetClusterUIInfo_Vtbl, 0x97dede50_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IGetClusterUIInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetClusterUIInfo, windows_core::IUnknown);
impl IGetClusterUIInfo {
    pub unsafe fn GetClusterName(&self, lpszname: &windows_core::BSTR, pcchname: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClusterName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lpszname), pcchname).ok()
    }
    pub unsafe fn GetLocale(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetLocale)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetFont(&self) -> super::super::Graphics::Gdi::HFONT {
        (windows_core::Interface::vtable(self).GetFont)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetIcon(&self) -> super::super::UI::WindowsAndMessaging::HICON {
        (windows_core::Interface::vtable(self).GetIcon)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IGetClusterUIInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClusterName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub GetLocale: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetFont: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Graphics::Gdi::HFONT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetFont: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetIcon: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::UI::WindowsAndMessaging::HICON,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetIcon: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusApplication, ISClusApplication_Vtbl, 0xf2e606e6_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusApplication {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusApplication, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusApplication {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DomainNames(&self) -> windows_core::Result<ISDomainNames> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DomainNames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_ClusterNames<P0>(&self, bstrdomainname: P0) -> windows_core::Result<ISClusterNames>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ClusterNames)(windows_core::Interface::as_raw(self), bstrdomainname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenCluster<P0>(&self, bstrclustername: P0) -> windows_core::Result<ISCluster>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenCluster)(windows_core::Interface::as_raw(self), bstrclustername.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusApplication_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DomainNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DomainNames: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ClusterNames: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ClusterNames: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenCluster: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenCluster: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusCryptoKeys, ISClusCryptoKeys_Vtbl, 0xf2e6072c_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusCryptoKeys {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusCryptoKeys, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusCryptoKeys {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddItem<P0>(&self, bstrcryptokey: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), bstrcryptokey.param().abi()).ok()
    }
    pub unsafe fn RemoveItem<P0>(&self, varindex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), varindex.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusCryptoKeys_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusDisk, ISClusDisk_Vtbl, 0xf2e60724_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusDisk {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusDisk, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusDisk {
    pub unsafe fn Signature(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Signature)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ScsiAddress(&self) -> windows_core::Result<ISClusScsiAddress> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ScsiAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DiskNumber(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DiskNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Partitions(&self) -> windows_core::Result<ISClusPartitions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Partitions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusDisk_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Signature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ScsiAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ScsiAddress: usize,
    pub DiskNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Partitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Partitions: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusDisks, ISClusDisks_Vtbl, 0xf2e60726_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusDisks {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusDisks, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusDisks {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusDisk>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusDisks_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusNetInterface, ISClusNetInterface_Vtbl, 0xf2e606ee_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusNetInterface {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusNetInterface, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetInterface {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommonProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommonROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn State(&self) -> windows_core::Result<CLUSTER_NETINTERFACE_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Cluster(&self) -> windows_core::Result<ISCluster> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cluster)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusNetInterface_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonROProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateROProperties: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_NETINTERFACE_STATE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Cluster: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cluster: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusNetInterfaces, ISClusNetInterfaces_Vtbl, 0xf2e606f0_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusNetInterfaces {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusNetInterfaces, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetInterfaces {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusNetInterface>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusNetInterfaces_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusNetwork, ISClusNetwork_Vtbl, 0xf2e606f2_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusNetwork {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusNetwork, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetwork {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommonProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommonROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, bstrnetworkname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), bstrnetworkname.param().abi()).ok()
    }
    pub unsafe fn NetworkID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NetworkID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn State(&self) -> windows_core::Result<CLUSTER_NETWORK_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NetInterfaces(&self) -> windows_core::Result<ISClusNetworkNetInterfaces> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NetInterfaces)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Cluster(&self) -> windows_core::Result<ISCluster> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cluster)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusNetwork_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonROProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateROProperties: usize,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub NetworkID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_NETWORK_STATE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NetInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NetInterfaces: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Cluster: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cluster: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusNetworkNetInterfaces, ISClusNetworkNetInterfaces_Vtbl, 0xf2e606f6_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusNetworkNetInterfaces {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusNetworkNetInterfaces, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetworkNetInterfaces {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusNetInterface>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusNetworkNetInterfaces_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusNetworks, ISClusNetworks_Vtbl, 0xf2e606f4_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusNetworks {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusNetworks, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetworks {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusNetwork>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusNetworks_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusNode, ISClusNode_Vtbl, 0xf2e606f8_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusNode {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusNode, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNode {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommonProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommonROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NodeID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NodeID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn State(&self) -> windows_core::Result<CLUSTER_NODE_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Evict(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Evict)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResourceGroups(&self) -> windows_core::Result<ISClusResGroups> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResourceGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Cluster(&self) -> windows_core::Result<ISCluster> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cluster)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NetInterfaces(&self) -> windows_core::Result<ISClusNodeNetInterfaces> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NetInterfaces)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusNode_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonROProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateROProperties: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub NodeID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_NODE_STATE) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Evict: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResourceGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResourceGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Cluster: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cluster: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NetInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NetInterfaces: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusNodeNetInterfaces, ISClusNodeNetInterfaces_Vtbl, 0xf2e606fc_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusNodeNetInterfaces {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusNodeNetInterfaces, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNodeNetInterfaces {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusNetInterface>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusNodeNetInterfaces_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusNodes, ISClusNodes_Vtbl, 0xf2e606fa_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusNodes {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusNodes, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNodes {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusNode>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusNodes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusPartition, ISClusPartition_Vtbl, 0xf2e60720_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusPartition {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusPartition, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPartition {
    pub unsafe fn Flags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeviceName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn VolumeLabel(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VolumeLabel)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SerialNumber(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SerialNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MaximumComponentLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaximumComponentLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FileSystemFlags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileSystemFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FileSystem(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileSystem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusPartition_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub VolumeLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaximumComponentLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FileSystemFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FileSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusPartitionEx, ISClusPartitionEx_Vtbl, 0x8802d4fe_b32e_4ad1_9dbd_64f18e1166ce);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusPartitionEx {
    type Target = ISClusPartition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusPartitionEx, windows_core::IUnknown, super::super::System::Com::IDispatch, ISClusPartition);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPartitionEx {
    pub unsafe fn TotalSize(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FreeSpace(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FreeSpace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DeviceNumber(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeviceNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PartitionNumber(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PartitionNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn VolumeGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VolumeGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusPartitionEx_Vtbl {
    pub base__: ISClusPartition_Vtbl,
    pub TotalSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FreeSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DeviceNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PartitionNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub VolumeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusPartitions, ISClusPartitions_Vtbl, 0xf2e60722_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusPartitions {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusPartitions, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPartitions {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusPartition>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusPartitions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusProperties, ISClusProperties_Vtbl, 0xf2e60700_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusProperties {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusProperties, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusProperties {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusProperty>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateItem<P0, P1>(&self, bstrname: P0, varvalue: P1) -> windows_core::Result<ISClusProperty>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), bstrname.param().abi(), varvalue.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UseDefaultValue<P0>(&self, varindex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).UseDefaultValue)(windows_core::Interface::as_raw(self), varindex.param().abi()).ok()
    }
    pub unsafe fn SaveChanges(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SaveChanges)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Private(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Private)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Common(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Common)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Modified(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Modified)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusProperties_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateItem: usize,
    pub UseDefaultValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SaveChanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Private: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Common: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Modified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusProperty, ISClusProperty_Vtbl, 0xf2e606fe_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusProperty {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusProperty, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusProperty {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Length(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ValueCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ValueCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Values(&self) -> windows_core::Result<ISClusPropertyValues> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Values)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue<P0>(&self, varvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), varvalue.param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> windows_core::Result<CLUSTER_PROPERTY_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetType(&self, r#type: CLUSTER_PROPERTY_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), r#type).ok()
    }
    pub unsafe fn Format(&self) -> windows_core::Result<CLUSTER_PROPERTY_FORMAT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Format)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFormat(&self, format: CLUSTER_PROPERTY_FORMAT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFormat)(windows_core::Interface::as_raw(self), format).ok()
    }
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Private(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Private)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Common(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Common)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Modified(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Modified)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UseDefaultValue(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UseDefaultValue)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusProperty_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ValueCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Values: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Values: usize,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_PROPERTY_TYPE) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, CLUSTER_PROPERTY_TYPE) -> windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_PROPERTY_FORMAT) -> windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, CLUSTER_PROPERTY_FORMAT) -> windows_core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Private: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Common: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Modified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub UseDefaultValue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusPropertyValue, ISClusPropertyValue_Vtbl, 0xf2e6071a_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusPropertyValue {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusPropertyValue, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertyValue {
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue<P0>(&self, varvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), varvalue.param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> windows_core::Result<CLUSTER_PROPERTY_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetType(&self, r#type: CLUSTER_PROPERTY_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), r#type).ok()
    }
    pub unsafe fn Format(&self) -> windows_core::Result<CLUSTER_PROPERTY_FORMAT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Format)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFormat(&self, format: CLUSTER_PROPERTY_FORMAT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFormat)(windows_core::Interface::as_raw(self), format).ok()
    }
    pub unsafe fn Length(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DataCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Data(&self) -> windows_core::Result<ISClusPropertyValueData> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Data)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusPropertyValue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_PROPERTY_TYPE) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, CLUSTER_PROPERTY_TYPE) -> windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_PROPERTY_FORMAT) -> windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, CLUSTER_PROPERTY_FORMAT) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DataCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Data: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusPropertyValueData, ISClusPropertyValueData_Vtbl, 0xf2e6071e_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusPropertyValueData {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusPropertyValueData, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertyValueData {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateItem<P0>(&self, varvalue: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), varvalue.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveItem<P0>(&self, varindex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), varindex.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusPropertyValueData_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusPropertyValues, ISClusPropertyValues_Vtbl, 0xf2e6071c_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusPropertyValues {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusPropertyValues, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertyValues {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusPropertyValue>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateItem<P0, P1>(&self, bstrname: P0, varvalue: P1) -> windows_core::Result<ISClusPropertyValue>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), bstrname.param().abi(), varvalue.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveItem<P0>(&self, varindex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), varindex.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusPropertyValues_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateItem: usize,
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusRefObject, ISClusRefObject_Vtbl, 0xf2e60702_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusRefObject {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusRefObject, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusRefObject {
    pub unsafe fn Handle(&self) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusRefObject_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusRegistryKeys, ISClusRegistryKeys_Vtbl, 0xf2e6072a_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusRegistryKeys {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusRegistryKeys, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusRegistryKeys {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddItem<P0>(&self, bstrregistrykey: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), bstrregistrykey.param().abi()).ok()
    }
    pub unsafe fn RemoveItem<P0>(&self, varindex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), varindex.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusRegistryKeys_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResDependencies, ISClusResDependencies_Vtbl, 0xf2e60704_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResDependencies {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResDependencies, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResDependencies {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusResource>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateItem<P0, P1>(&self, bstrresourcename: P0, bstrresourcetype: P1, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), bstrresourcename.param().abi(), bstrresourcetype.param().abi(), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteItem<P0>(&self, varindex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), varindex.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddItem<P0>(&self, presource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusResource>,
    {
        (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), presource.param().abi()).ok()
    }
    pub unsafe fn RemoveItem<P0>(&self, varindex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), varindex.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusResDependencies_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, CLUSTER_RESOURCE_CREATE_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateItem: usize,
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddItem: usize,
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResDependents, ISClusResDependents_Vtbl, 0xf2e6072e_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResDependents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResDependents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResDependents {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusResource>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateItem<P0, P1>(&self, bstrresourcename: P0, bstrresourcetype: P1, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), bstrresourcename.param().abi(), bstrresourcetype.param().abi(), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteItem<P0>(&self, varindex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), varindex.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddItem<P0>(&self, presource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusResource>,
    {
        (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), presource.param().abi()).ok()
    }
    pub unsafe fn RemoveItem<P0>(&self, varindex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), varindex.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusResDependents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, CLUSTER_RESOURCE_CREATE_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateItem: usize,
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddItem: usize,
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResGroup, ISClusResGroup_Vtbl, 0xf2e60706_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResGroup {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResGroup, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroup {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommonProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommonROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, bstrgroupname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), bstrgroupname.param().abi()).ok()
    }
    pub unsafe fn State(&self) -> windows_core::Result<CLUSTER_GROUP_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OwnerNode(&self) -> windows_core::Result<ISClusNode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OwnerNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Resources(&self) -> windows_core::Result<ISClusResGroupResources> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Resources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PreferredOwnerNodes(&self) -> windows_core::Result<ISClusResGroupPreferredOwnerNodes> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PreferredOwnerNodes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Online<P0, P1>(&self, vartimeout: P0, varnode: P1) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Online)(windows_core::Interface::as_raw(self), vartimeout.param().abi(), varnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Move<P0, P1>(&self, vartimeout: P0, varnode: P1) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), vartimeout.param().abi(), varnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Offline<P0>(&self, vartimeout: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Offline)(windows_core::Interface::as_raw(self), vartimeout.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Cluster(&self) -> windows_core::Result<ISCluster> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cluster)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusResGroup_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonROProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateROProperties: usize,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_GROUP_STATE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OwnerNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OwnerNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Resources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Resources: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PreferredOwnerNodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PreferredOwnerNodes: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Online: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Offline: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Cluster: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cluster: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResGroupPreferredOwnerNodes, ISClusResGroupPreferredOwnerNodes_Vtbl, 0xf2e606e8_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResGroupPreferredOwnerNodes {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResGroupPreferredOwnerNodes, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroupPreferredOwnerNodes {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusNode>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertItem<P0>(&self, pnode: P0, nposition: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusNode>,
    {
        (windows_core::Interface::vtable(self).InsertItem)(windows_core::Interface::as_raw(self), pnode.param().abi(), nposition).ok()
    }
    pub unsafe fn RemoveItem<P0>(&self, varindex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), varindex.param().abi()).ok()
    }
    pub unsafe fn Modified(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Modified)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SaveChanges(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SaveChanges)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddItem<P0>(&self, pnode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusNode>,
    {
        (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), pnode.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusResGroupPreferredOwnerNodes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertItem: usize,
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Modified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SaveChanges: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddItem: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResGroupResources, ISClusResGroupResources_Vtbl, 0xf2e606ea_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResGroupResources {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResGroupResources, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroupResources {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusResource>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateItem<P0, P1>(&self, bstrresourcename: P0, bstrresourcetype: P1, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), bstrresourcename.param().abi(), bstrresourcetype.param().abi(), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteItem<P0>(&self, varindex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), varindex.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusResGroupResources_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, CLUSTER_RESOURCE_CREATE_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateItem: usize,
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResGroups, ISClusResGroups_Vtbl, 0xf2e60708_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResGroups {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResGroups, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroups {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusResGroup>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateItem<P0>(&self, bstrresourcegroupname: P0) -> windows_core::Result<ISClusResGroup>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), bstrresourcegroupname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteItem<P0>(&self, varindex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), varindex.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusResGroups_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateItem: usize,
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResPossibleOwnerNodes, ISClusResPossibleOwnerNodes_Vtbl, 0xf2e6070e_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResPossibleOwnerNodes {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResPossibleOwnerNodes, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResPossibleOwnerNodes {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusNode>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddItem<P0>(&self, pnode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusNode>,
    {
        (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), pnode.param().abi()).ok()
    }
    pub unsafe fn RemoveItem<P0>(&self, varindex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), varindex.param().abi()).ok()
    }
    pub unsafe fn Modified(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Modified)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusResPossibleOwnerNodes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddItem: usize,
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Modified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResType, ISClusResType_Vtbl, 0xf2e60710_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResType {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResType, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResType {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommonProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommonROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Cluster(&self) -> windows_core::Result<ISCluster> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cluster)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Resources(&self) -> windows_core::Result<ISClusResTypeResources> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Resources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PossibleOwnerNodes(&self) -> windows_core::Result<ISClusResTypePossibleOwnerNodes> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PossibleOwnerNodes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AvailableDisks(&self) -> windows_core::Result<ISClusDisks> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AvailableDisks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusResType_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonROProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateROProperties: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Cluster: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cluster: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Resources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Resources: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PossibleOwnerNodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PossibleOwnerNodes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AvailableDisks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AvailableDisks: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResTypePossibleOwnerNodes, ISClusResTypePossibleOwnerNodes_Vtbl, 0xf2e60718_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResTypePossibleOwnerNodes {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResTypePossibleOwnerNodes, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResTypePossibleOwnerNodes {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusNode>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusResTypePossibleOwnerNodes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResTypeResources, ISClusResTypeResources_Vtbl, 0xf2e60714_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResTypeResources {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResTypeResources, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResTypeResources {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusResource>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateItem<P0, P1>(&self, bstrresourcename: P0, bstrgroupname: P1, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), bstrresourcename.param().abi(), bstrgroupname.param().abi(), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteItem<P0>(&self, varindex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), varindex.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusResTypeResources_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, CLUSTER_RESOURCE_CREATE_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateItem: usize,
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResTypes, ISClusResTypes_Vtbl, 0xf2e60712_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResTypes {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResTypes, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResTypes {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusResType>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateItem<P0, P1, P2>(&self, bstrresourcetypename: P0, bstrdisplayname: P1, bstrresourcetypedll: P2, dwlooksalivepollinterval: i32, dwisalivepollinterval: i32) -> windows_core::Result<ISClusResType>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), bstrresourcetypename.param().abi(), bstrdisplayname.param().abi(), bstrresourcetypedll.param().abi(), dwlooksalivepollinterval, dwisalivepollinterval, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteItem<P0>(&self, varindex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), varindex.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusResTypes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateItem: usize,
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResource, ISClusResource_Vtbl, 0xf2e6070a_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResource {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResource, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResource {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommonProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommonROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, bstrresourcename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), bstrresourcename.param().abi()).ok()
    }
    pub unsafe fn State(&self) -> windows_core::Result<CLUSTER_RESOURCE_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CoreFlag(&self) -> windows_core::Result<CLUS_FLAGS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CoreFlag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn BecomeQuorumResource<P0>(&self, bstrdevicepath: P0, lmaxlogsize: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).BecomeQuorumResource)(windows_core::Interface::as_raw(self), bstrdevicepath.param().abi(), lmaxlogsize).ok()
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Fail(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Fail)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Online(&self, ntimeout: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Online)(windows_core::Interface::as_raw(self), ntimeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Offline(&self, ntimeout: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Offline)(windows_core::Interface::as_raw(self), ntimeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ChangeResourceGroup<P0>(&self, presourcegroup: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusResGroup>,
    {
        (windows_core::Interface::vtable(self).ChangeResourceGroup)(windows_core::Interface::as_raw(self), presourcegroup.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddResourceNode<P0>(&self, pnode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusNode>,
    {
        (windows_core::Interface::vtable(self).AddResourceNode)(windows_core::Interface::as_raw(self), pnode.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoveResourceNode<P0>(&self, pnode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusNode>,
    {
        (windows_core::Interface::vtable(self).RemoveResourceNode)(windows_core::Interface::as_raw(self), pnode.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CanResourceBeDependent<P0>(&self, presource: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<ISClusResource>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanResourceBeDependent)(windows_core::Interface::as_raw(self), presource.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PossibleOwnerNodes(&self) -> windows_core::Result<ISClusResPossibleOwnerNodes> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PossibleOwnerNodes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Dependencies(&self) -> windows_core::Result<ISClusResDependencies> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Dependencies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Dependents(&self) -> windows_core::Result<ISClusResDependents> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Dependents)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Group(&self) -> windows_core::Result<ISClusResGroup> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Group)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OwnerNode(&self) -> windows_core::Result<ISClusNode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OwnerNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Cluster(&self) -> windows_core::Result<ISCluster> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cluster)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ClassInfo(&self) -> windows_core::Result<CLUSTER_RESOURCE_CLASS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClassInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Disk(&self) -> windows_core::Result<ISClusDisk> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Disk)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegistryKeys(&self) -> windows_core::Result<ISClusRegistryKeys> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegistryKeys)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CryptoKeys(&self) -> windows_core::Result<ISClusCryptoKeys> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CryptoKeys)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TypeName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TypeName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Type(&self) -> windows_core::Result<ISClusResType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MaintenanceMode(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaintenanceMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaintenanceMode<P0>(&self, bmaintenancemode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetMaintenanceMode)(windows_core::Interface::as_raw(self), bmaintenancemode.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusResource_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonROProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateROProperties: usize,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_RESOURCE_STATE) -> windows_core::HRESULT,
    pub CoreFlag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUS_FLAGS) -> windows_core::HRESULT,
    pub BecomeQuorumResource: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Fail: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Online: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Offline: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ChangeResourceGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ChangeResourceGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddResourceNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddResourceNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveResourceNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveResourceNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CanResourceBeDependent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CanResourceBeDependent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PossibleOwnerNodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PossibleOwnerNodes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Dependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Dependencies: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Dependents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Dependents: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Group: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OwnerNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OwnerNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Cluster: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cluster: usize,
    pub ClassInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_RESOURCE_CLASS) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Disk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Disk: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RegistryKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegistryKeys: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CryptoKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CryptoKeys: usize,
    pub TypeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Type: usize,
    pub MaintenanceMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetMaintenanceMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResources, ISClusResources_Vtbl, 0xf2e6070c_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResources {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResources, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResources {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<ISClusResource>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateItem<P0, P1, P2>(&self, bstrresourcename: P0, bstrresourcetype: P1, bstrgroupname: P2, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), bstrresourcename.param().abi(), bstrresourcetype.param().abi(), bstrgroupname.param().abi(), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteItem<P0>(&self, varindex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), varindex.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusResources_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, CLUSTER_RESOURCE_CREATE_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateItem: usize,
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusScsiAddress, ISClusScsiAddress_Vtbl, 0xf2e60728_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusScsiAddress {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusScsiAddress, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusScsiAddress {
    pub unsafe fn PortNumber(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PortNumber)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PathId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PathId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TargetId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TargetId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Lun(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Lun)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusScsiAddress_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub PortNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PathId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub TargetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Lun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusVersion, ISClusVersion_Vtbl, 0xf2e60716_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusVersion {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusVersion, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusVersion {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MajorVersion(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MajorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MinorVersion(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MinorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn BuildNumber(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BuildNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn VendorId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VendorId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CSDVersion(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CSDVersion)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ClusterHighestVersion(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClusterHighestVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ClusterLowestVersion(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClusterLowestVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MixedVersion(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MixedVersion)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusVersion_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub BuildNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub VendorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CSDVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClusterHighestVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ClusterLowestVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MixedVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISCluster, ISCluster_Vtbl, 0xf2e606e4_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISCluster {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISCluster, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISCluster {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommonProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommonROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Open<P0>(&self, bstrclustername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), bstrclustername.param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, bstrclustername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), bstrclustername.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Version(&self) -> windows_core::Result<ISClusVersion> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetQuorumResource<P0>(&self, pclusterresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusResource>,
    {
        (windows_core::Interface::vtable(self).SetQuorumResource)(windows_core::Interface::as_raw(self), pclusterresource.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QuorumResource(&self) -> windows_core::Result<ISClusResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuorumResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QuorumLogSize(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuorumLogSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQuorumLogSize(&self, nlogsize: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQuorumLogSize)(windows_core::Interface::as_raw(self), nlogsize).ok()
    }
    pub unsafe fn QuorumPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuorumPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetQuorumPath<P0>(&self, ppath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetQuorumPath)(windows_core::Interface::as_raw(self), ppath.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Nodes(&self) -> windows_core::Result<ISClusNodes> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Nodes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResourceGroups(&self) -> windows_core::Result<ISClusResGroups> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResourceGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Resources(&self) -> windows_core::Result<ISClusResources> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Resources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResourceTypes(&self) -> windows_core::Result<ISClusResTypes> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResourceTypes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Networks(&self) -> windows_core::Result<ISClusNetworks> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Networks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NetInterfaces(&self) -> windows_core::Result<ISClusNetInterfaces> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NetInterfaces)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISCluster_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommonROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommonROProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateROProperties: usize,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Version: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetQuorumResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetQuorumResource: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub QuorumResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QuorumResource: usize,
    pub QuorumLogSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetQuorumLogSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub QuorumPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetQuorumPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Nodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Nodes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ResourceGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResourceGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Resources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Resources: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ResourceTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResourceTypes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Networks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Networks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NetInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NetInterfaces: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusterNames, ISClusterNames_Vtbl, 0xf2e606ec_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusterNames {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusterNames, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusterNames {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DomainName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DomainName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISClusterNames_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DomainName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISDomainNames, ISDomainNames_Vtbl, 0xf2e606e2_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISDomainNames {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISDomainNames, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISDomainNames {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn get_Item<P0>(&self, varindex: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISDomainNames_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWCContextMenuCallback, IWCContextMenuCallback_Vtbl, 0x97dede64_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IWCContextMenuCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWCContextMenuCallback, windows_core::IUnknown);
impl IWCContextMenuCallback {
    pub unsafe fn AddExtensionMenuItem<P0, P1>(&self, lpszname: P0, lpszstatusbartext: P1, ncommandid: u32, nsubmenucommandid: u32, uflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddExtensionMenuItem)(windows_core::Interface::as_raw(self), lpszname.param().abi(), lpszstatusbartext.param().abi(), ncommandid, nsubmenucommandid, uflags).ok()
    }
}
#[repr(C)]
pub struct IWCContextMenuCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddExtensionMenuItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, u32, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWCPropertySheetCallback, IWCPropertySheetCallback_Vtbl, 0x97dede60_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IWCPropertySheetCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWCPropertySheetCallback, windows_core::IUnknown);
impl IWCPropertySheetCallback {
    pub unsafe fn AddPropertySheetPage(&self, hpage: *const i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddPropertySheetPage)(windows_core::Interface::as_raw(self), hpage).ok()
    }
}
#[repr(C)]
pub struct IWCPropertySheetCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddPropertySheetPage: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWCWizard97Callback, IWCWizard97Callback_Vtbl, 0x97dede67_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IWCWizard97Callback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWCWizard97Callback, windows_core::IUnknown);
impl IWCWizard97Callback {
    pub unsafe fn AddWizard97Page(&self, hpage: *const i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddWizard97Page)(windows_core::Interface::as_raw(self), hpage).ok()
    }
    pub unsafe fn EnableNext<P0>(&self, hpage: *const i32, benable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).EnableNext)(windows_core::Interface::as_raw(self), hpage, benable.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWCWizard97Callback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddWizard97Page: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32) -> windows_core::HRESULT,
    pub EnableNext: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWCWizardCallback, IWCWizardCallback_Vtbl, 0x97dede62_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IWCWizardCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWCWizardCallback, windows_core::IUnknown);
impl IWCWizardCallback {
    pub unsafe fn AddWizardPage(&self, hpage: *const i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddWizardPage)(windows_core::Interface::as_raw(self), hpage).ok()
    }
    pub unsafe fn EnableNext<P0>(&self, hpage: *const i32, benable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).EnableNext)(windows_core::Interface::as_raw(self), hpage, benable.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWCWizardCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddWizardPage: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32) -> windows_core::HRESULT,
    pub EnableNext: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWEExtendContextMenu, IWEExtendContextMenu_Vtbl, 0x97dede65_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IWEExtendContextMenu {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWEExtendContextMenu, windows_core::IUnknown);
impl IWEExtendContextMenu {
    pub unsafe fn AddContextMenuItems<P0, P1>(&self, pidata: P0, picallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<IWCContextMenuCallback>,
    {
        (windows_core::Interface::vtable(self).AddContextMenuItems)(windows_core::Interface::as_raw(self), pidata.param().abi(), picallback.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWEExtendContextMenu_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddContextMenuItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWEExtendPropertySheet, IWEExtendPropertySheet_Vtbl, 0x97dede61_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IWEExtendPropertySheet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWEExtendPropertySheet, windows_core::IUnknown);
impl IWEExtendPropertySheet {
    pub unsafe fn CreatePropertySheetPages<P0, P1>(&self, pidata: P0, picallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<IWCPropertySheetCallback>,
    {
        (windows_core::Interface::vtable(self).CreatePropertySheetPages)(windows_core::Interface::as_raw(self), pidata.param().abi(), picallback.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWEExtendPropertySheet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreatePropertySheetPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWEExtendWizard, IWEExtendWizard_Vtbl, 0x97dede63_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IWEExtendWizard {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWEExtendWizard, windows_core::IUnknown);
impl IWEExtendWizard {
    pub unsafe fn CreateWizardPages<P0, P1>(&self, pidata: P0, picallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<IWCWizardCallback>,
    {
        (windows_core::Interface::vtable(self).CreateWizardPages)(windows_core::Interface::as_raw(self), pidata.param().abi(), picallback.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWEExtendWizard_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateWizardPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWEExtendWizard97, IWEExtendWizard97_Vtbl, 0x97dede68_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IWEExtendWizard97 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWEExtendWizard97, windows_core::IUnknown);
impl IWEExtendWizard97 {
    pub unsafe fn CreateWizard97Pages<P0, P1>(&self, pidata: P0, picallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<IWCWizard97Callback>,
    {
        (windows_core::Interface::vtable(self).CreateWizard97Pages)(windows_core::Interface::as_raw(self), pidata.param().abi(), picallback.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWEExtendWizard97_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateWizard97Pages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWEInvokeCommand, IWEInvokeCommand_Vtbl, 0x97dede66_fc6b_11cf_b5f5_00a0c90ab505);
impl core::ops::Deref for IWEInvokeCommand {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWEInvokeCommand, windows_core::IUnknown);
impl IWEInvokeCommand {
    pub unsafe fn InvokeCommand<P0>(&self, ncommandid: u32, pidata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).InvokeCommand)(windows_core::Interface::as_raw(self), ncommandid, pidata.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWEInvokeCommand_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InvokeCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const BitLockerDecrypted: i32 = 4i32;
pub const BitLockerDecrypting: i32 = 16i32;
pub const BitLockerEnabled: i32 = 1i32;
pub const BitLockerPaused: i32 = 64i32;
pub const BitLockerStopped: i32 = 128i32;
pub const BitlockerEncrypted: i32 = 8i32;
pub const BitlockerEncrypting: i32 = 32i32;
pub const CA_UPGRADE_VERSION: u32 = 1u32;
pub const CLCTL_ADD_CRYPTO_CHECKPOINT: CLCTL_CODES = CLCTL_CODES(4194478i32);
pub const CLCTL_ADD_CRYPTO_CHECKPOINT_EX: CLCTL_CODES = CLCTL_CODES(4195030i32);
pub const CLCTL_ADD_DEPENDENCY: CLCTL_CODES = CLCTL_CODES(5242898i32);
pub const CLCTL_ADD_OWNER: CLCTL_CODES = CLCTL_CODES(5242906i32);
pub const CLCTL_ADD_REGISTRY_CHECKPOINT: CLCTL_CODES = CLCTL_CODES(4194466i32);
pub const CLCTL_ADD_REGISTRY_CHECKPOINT_32BIT: CLCTL_CODES = CLCTL_CODES(4194498i32);
pub const CLCTL_ADD_REGISTRY_CHECKPOINT_64BIT: CLCTL_CODES = CLCTL_CODES(4194494i32);
pub const CLCTL_BATCH_BLOCK_KEY: CLCTL_CODES = CLCTL_CODES(574i32);
pub const CLCTL_BATCH_UNBLOCK_KEY: CLCTL_CODES = CLCTL_CODES(577i32);
pub const CLCTL_BLOCK_GEM_SEND_RECV: CLCTL_CODES = CLCTL_CODES(717i32);
pub const CLCTL_CHECK_DRAIN_VETO: CLCTL_CODES = CLCTL_CODES(1057069i32);
pub const CLCTL_CHECK_VOTER_DOWN: CLCTL_CODES = CLCTL_CODES(73i32);
pub const CLCTL_CHECK_VOTER_DOWN_WITNESS: CLCTL_CODES = CLCTL_CODES(113i32);
pub const CLCTL_CHECK_VOTER_EVICT: CLCTL_CODES = CLCTL_CODES(69i32);
pub const CLCTL_CHECK_VOTER_EVICT_WITNESS: CLCTL_CODES = CLCTL_CODES(109i32);
pub const CLCTL_CLEAR_NODE_CONNECTION_INFO: CLCTL_CODES = CLCTL_CODES(4195078i32);
pub const CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS: CLCTL_CODES = CLCTL_CODES(8417i32);
pub const CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY: CLCTL_CODES = CLCTL_CODES(8433i32);
pub const CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY: CLCTL_CODES = CLCTL_CODES(4202742i32);
pub const CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN: CLCTL_CODES = CLCTL_CODES(4202726i32);
pub const CLCTL_CLUSTER_BASE: u32 = 0u32;
pub const CLCTL_CLUSTER_NAME_CHANGED: CLCTL_CODES = CLCTL_CODES(5242922i32);
pub const CLCTL_CLUSTER_VERSION_CHANGED: CLCTL_CODES = CLCTL_CODES(5242926i32);
pub const CLCTL_DELETE: CLCTL_CODES = CLCTL_CODES(5242886i32);
pub const CLCTL_DELETE_CRYPTO_CHECKPOINT: CLCTL_CODES = CLCTL_CODES(4194482i32);
pub const CLCTL_DELETE_REGISTRY_CHECKPOINT: CLCTL_CODES = CLCTL_CODES(4194470i32);
pub const CLCTL_DISABLE_SHARED_VOLUME_DIRECTIO: CLCTL_CODES = CLCTL_CODES(4194958i32);
pub const CLCTL_ENABLE_SHARED_VOLUME_DIRECTIO: CLCTL_CODES = CLCTL_CODES(4194954i32);
pub const CLCTL_ENUM_AFFINITY_RULE_NAMES: CLCTL_CODES = CLCTL_CODES(11741i32);
pub const CLCTL_ENUM_COMMON_PROPERTIES: CLCTL_CODES = CLCTL_CODES(81i32);
pub const CLCTL_ENUM_PRIVATE_PROPERTIES: CLCTL_CODES = CLCTL_CODES(121i32);
pub const CLCTL_EVICT_NODE: CLCTL_CODES = CLCTL_CODES(5242894i32);
pub const CLCTL_FILESERVER_SHARE_ADD: CLCTL_CODES = CLCTL_CODES(4194886i32);
pub const CLCTL_FILESERVER_SHARE_DEL: CLCTL_CODES = CLCTL_CODES(4194890i32);
pub const CLCTL_FILESERVER_SHARE_MODIFY: CLCTL_CODES = CLCTL_CODES(4194894i32);
pub const CLCTL_FILESERVER_SHARE_REPORT: CLCTL_CODES = CLCTL_CODES(593i32);
pub const CLCTL_FIXUP_ON_UPGRADE: CLCTL_CODES = CLCTL_CODES(5242930i32);
pub const CLCTL_FORCE_DB_FLUSH: CLCTL_CODES = CLCTL_CODES(4206054i32);
pub const CLCTL_FORCE_QUORUM: CLCTL_CODES = CLCTL_CODES(5242950i32);
pub const CLCTL_FSWITNESS_GET_EPOCH_INFO: CLCTL_CODES = CLCTL_CODES(1048669i32);
pub const CLCTL_FSWITNESS_RELEASE_LOCK: CLCTL_CODES = CLCTL_CODES(5242982i32);
pub const CLCTL_FSWITNESS_SET_EPOCH_INFO: CLCTL_CODES = CLCTL_CODES(5242978i32);
pub const CLCTL_GET_ARB_TIMEOUT: CLCTL_CODES = CLCTL_CODES(21i32);
pub const CLCTL_GET_CHARACTERISTICS: CLCTL_CODES = CLCTL_CODES(5i32);
pub const CLCTL_GET_CLASS_INFO: CLCTL_CODES = CLCTL_CODES(13i32);
pub const CLCTL_GET_CLUSDB_TIMESTAMP: CLCTL_CODES = CLCTL_CODES(681i32);
pub const CLCTL_GET_CLUSTER_SERVICE_ACCOUNT_NAME: CLCTL_CODES = CLCTL_CODES(65i32);
pub const CLCTL_GET_COMMON_PROPERTIES: CLCTL_CODES = CLCTL_CODES(89i32);
pub const CLCTL_GET_COMMON_PROPERTY_FMTS: CLCTL_CODES = CLCTL_CODES(101i32);
pub const CLCTL_GET_COMMON_RESOURCE_PROPERTY_FMTS: CLCTL_CODES = CLCTL_CODES(105i32);
pub const CLCTL_GET_CRYPTO_CHECKPOINTS: CLCTL_CODES = CLCTL_CODES(181i32);
pub const CLCTL_GET_DNS_NAME: CLCTL_CODES = CLCTL_CODES(373i32);
pub const CLCTL_GET_FAILURE_INFO: CLCTL_CODES = CLCTL_CODES(25i32);
pub const CLCTL_GET_FLAGS: CLCTL_CODES = CLCTL_CODES(9i32);
pub const CLCTL_GET_FQDN: CLCTL_CODES = CLCTL_CODES(61i32);
pub const CLCTL_GET_GEMID_VECTOR: CLCTL_CODES = CLCTL_CODES(721i32);
pub const CLCTL_GET_GUM_LOCK_OWNER: CLCTL_CODES = CLCTL_CODES(697i32);
pub const CLCTL_GET_ID: CLCTL_CODES = CLCTL_CODES(57i32);
pub const CLCTL_GET_INFRASTRUCTURE_SOFS_BUFFER: CLCTL_CODES = CLCTL_CODES(11657i32);
pub const CLCTL_GET_LOADBAL_PROCESS_LIST: CLCTL_CODES = CLCTL_CODES(201i32);
pub const CLCTL_GET_NAME: CLCTL_CODES = CLCTL_CODES(41i32);
pub const CLCTL_GET_NETWORK: CLCTL_CODES = CLCTL_CODES(53i32);
pub const CLCTL_GET_NETWORK_NAME: CLCTL_CODES = CLCTL_CODES(361i32);
pub const CLCTL_GET_NODE: CLCTL_CODES = CLCTL_CODES(49i32);
pub const CLCTL_GET_NODES_IN_FD: CLCTL_CODES = CLCTL_CODES(11745i32);
pub const CLCTL_GET_OPERATION_CONTEXT: CLCTL_CODES = CLCTL_CODES(1057001i32);
pub const CLCTL_GET_PRIVATE_PROPERTIES: CLCTL_CODES = CLCTL_CODES(129i32);
pub const CLCTL_GET_PRIVATE_PROPERTY_FMTS: CLCTL_CODES = CLCTL_CODES(141i32);
pub const CLCTL_GET_PRIVATE_RESOURCE_PROPERTY_FMTS: CLCTL_CODES = CLCTL_CODES(145i32);
pub const CLCTL_GET_REGISTRY_CHECKPOINTS: CLCTL_CODES = CLCTL_CODES(169i32);
pub const CLCTL_GET_REQUIRED_DEPENDENCIES: CLCTL_CODES = CLCTL_CODES(17i32);
pub const CLCTL_GET_RESOURCE_TYPE: CLCTL_CODES = CLCTL_CODES(45i32);
pub const CLCTL_GET_RO_COMMON_PROPERTIES: CLCTL_CODES = CLCTL_CODES(85i32);
pub const CLCTL_GET_RO_PRIVATE_PROPERTIES: CLCTL_CODES = CLCTL_CODES(125i32);
pub const CLCTL_GET_SHARED_VOLUME_ID: CLCTL_CODES = CLCTL_CODES(657i32);
pub const CLCTL_GET_STATE_CHANGE_TIME: CLCTL_CODES = CLCTL_CODES(11613i32);
pub const CLCTL_GET_STORAGE_CONFIGURATION: CLCTL_CODES = CLCTL_CODES(741i32);
pub const CLCTL_GET_STORAGE_CONFIG_ATTRIBUTES: CLCTL_CODES = CLCTL_CODES(745i32);
pub const CLCTL_GET_STUCK_NODES: CLCTL_CODES = CLCTL_CODES(701i32);
pub const CLCTL_GLOBAL_SHIFT: u32 = 23u32;
pub const CLCTL_GROUPSET_GET_GROUPS: CLCTL_CODES = CLCTL_CODES(11633i32);
pub const CLCTL_GROUPSET_GET_PROVIDER_GROUPS: CLCTL_CODES = CLCTL_CODES(11637i32);
pub const CLCTL_GROUPSET_GET_PROVIDER_GROUPSETS: CLCTL_CODES = CLCTL_CODES(11641i32);
pub const CLCTL_GROUP_GET_LAST_MOVE_TIME: CLCTL_CODES = CLCTL_CODES(729i32);
pub const CLCTL_GROUP_GET_PROVIDER_GROUPS: CLCTL_CODES = CLCTL_CODES(11645i32);
pub const CLCTL_GROUP_GET_PROVIDER_GROUPSETS: CLCTL_CODES = CLCTL_CODES(11649i32);
pub const CLCTL_GROUP_SET_CCF_FROM_MASTER: CLCTL_CODES = CLCTL_CODES(4205958i32);
pub const CLCTL_HOLD_IO: CLCTL_CODES = CLCTL_CODES(5242942i32);
pub const CLCTL_INITIALIZE: CLCTL_CODES = CLCTL_CODES(5242954i32);
pub const CLCTL_INJECT_GEM_FAULT: CLCTL_CODES = CLCTL_CODES(705i32);
pub const CLCTL_INSTALL_NODE: CLCTL_CODES = CLCTL_CODES(5242890i32);
pub const CLCTL_INTERNAL_SHIFT: u32 = 20u32;
pub const CLCTL_INTRODUCE_GEM_REPAIR_DELAY: CLCTL_CODES = CLCTL_CODES(709i32);
pub const CLCTL_IPADDRESS_RELEASE_LEASE: CLCTL_CODES = CLCTL_CODES(4194754i32);
pub const CLCTL_IPADDRESS_RENEW_LEASE: CLCTL_CODES = CLCTL_CODES(4194750i32);
pub const CLCTL_IS_FEATURE_INSTALLED: CLCTL_CODES = CLCTL_CODES(753i32);
pub const CLCTL_IS_QUORUM_BLOCKED: CLCTL_CODES = CLCTL_CODES(689i32);
pub const CLCTL_IS_S2D_FEATURE_SUPPORTED: CLCTL_CODES = CLCTL_CODES(757i32);
pub const CLCTL_JOINING_GROUP: CLCTL_CODES = CLCTL_CODES(5242970i32);
pub const CLCTL_LEAVING_GROUP: CLCTL_CODES = CLCTL_CODES(5242966i32);
pub const CLCTL_MODIFY_SHIFT: u32 = 22u32;
pub const CLCTL_NETNAME_CREDS_NOTIFYCAM: CLCTL_CODES = CLCTL_CODES(5242986i32);
pub const CLCTL_NETNAME_DELETE_CO: CLCTL_CODES = CLCTL_CODES(382i32);
pub const CLCTL_NETNAME_GET_OU_FOR_VCO: CLCTL_CODES = CLCTL_CODES(4194926i32);
pub const CLCTL_NETNAME_GET_VIRTUAL_SERVER_TOKEN: CLCTL_CODES = CLCTL_CODES(365i32);
pub const CLCTL_NETNAME_REGISTER_DNS_RECORDS: CLCTL_CODES = CLCTL_CODES(370i32);
pub const CLCTL_NETNAME_REPAIR_VCO: CLCTL_CODES = CLCTL_CODES(397i32);
pub const CLCTL_NETNAME_RESET_VCO: CLCTL_CODES = CLCTL_CODES(389i32);
pub const CLCTL_NETNAME_SET_PWD_INFO: CLCTL_CODES = CLCTL_CODES(378i32);
pub const CLCTL_NETNAME_SET_PWD_INFOEX: CLCTL_CODES = CLCTL_CODES(794i32);
pub const CLCTL_NETNAME_VALIDATE_VCO: CLCTL_CODES = CLCTL_CODES(385i32);
pub const CLCTL_NOTIFY_DRAIN_COMPLETE: CLCTL_CODES = CLCTL_CODES(1057073i32);
pub const CLCTL_NOTIFY_INFRASTRUCTURE_SOFS_CHANGED: CLCTL_CODES = CLCTL_CODES(4205970i32);
pub const CLCTL_NOTIFY_MONITOR_SHUTTING_DOWN: CLCTL_CODES = CLCTL_CODES(1048705i32);
pub const CLCTL_NOTIFY_OWNER_CHANGE: CLCTL_CODES = CLCTL_CODES(5251362i32);
pub const CLCTL_NOTIFY_QUORUM_STATUS: CLCTL_CODES = CLCTL_CODES(5243006i32);
pub const CLCTL_POOL_GET_DRIVE_INFO: CLCTL_CODES = CLCTL_CODES(693i32);
pub const CLCTL_PROVIDER_STATE_CHANGE: CLCTL_CODES = CLCTL_CODES(5242962i32);
pub const CLCTL_QUERY_DELETE: CLCTL_CODES = CLCTL_CODES(441i32);
pub const CLCTL_QUERY_MAINTENANCE_MODE: CLCTL_CODES = CLCTL_CODES(481i32);
pub const CLCTL_RELOAD_AUTOLOGGER_CONFIG: CLCTL_CODES = CLCTL_CODES(11730i32);
pub const CLCTL_REMOVE_DEPENDENCY: CLCTL_CODES = CLCTL_CODES(5242902i32);
pub const CLCTL_REMOVE_NODE: CLCTL_CODES = CLCTL_CODES(4195054i32);
pub const CLCTL_REMOVE_OWNER: CLCTL_CODES = CLCTL_CODES(5242910i32);
pub const CLCTL_REPLICATION_ADD_REPLICATION_GROUP: CLCTL_CODES = CLCTL_CODES(8514i32);
pub const CLCTL_REPLICATION_GET_ELIGIBLE_LOGDISKS: CLCTL_CODES = CLCTL_CODES(8521i32);
pub const CLCTL_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS: CLCTL_CODES = CLCTL_CODES(8529i32);
pub const CLCTL_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS: CLCTL_CODES = CLCTL_CODES(8525i32);
pub const CLCTL_REPLICATION_GET_LOG_INFO: CLCTL_CODES = CLCTL_CODES(8517i32);
pub const CLCTL_REPLICATION_GET_LOG_VOLUME: CLCTL_CODES = CLCTL_CODES(8541i32);
pub const CLCTL_REPLICATION_GET_REPLICATED_DISKS: CLCTL_CODES = CLCTL_CODES(8533i32);
pub const CLCTL_REPLICATION_GET_REPLICATED_PARTITION_INFO: CLCTL_CODES = CLCTL_CODES(8549i32);
pub const CLCTL_REPLICATION_GET_REPLICA_VOLUMES: CLCTL_CODES = CLCTL_CODES(8537i32);
pub const CLCTL_REPLICATION_GET_RESOURCE_GROUP: CLCTL_CODES = CLCTL_CODES(8545i32);
pub const CLCTL_RESOURCE_PREPARE_UPGRADE: CLCTL_CODES = CLCTL_CODES(4202730i32);
pub const CLCTL_RESOURCE_UPGRADE_COMPLETED: CLCTL_CODES = CLCTL_CODES(4202734i32);
pub const CLCTL_RESOURCE_UPGRADE_DLL: CLCTL_CODES = CLCTL_CODES(4194490i32);
pub const CLCTL_RESUME_IO: CLCTL_CODES = CLCTL_CODES(5242946i32);
pub const CLCTL_RW_MODIFY_NOOP: CLCTL_CODES = CLCTL_CODES(4194990i32);
pub const CLCTL_SCALEOUT_COMMAND: CLCTL_CODES = CLCTL_CODES(4205974i32);
pub const CLCTL_SCALEOUT_CONTROL: CLCTL_CODES = CLCTL_CODES(4205978i32);
pub const CLCTL_SCALEOUT_GET_CLUSTERS: CLCTL_CODES = CLCTL_CODES(4205981i32);
pub const CLCTL_SEND_DUMMY_GEM_MESSAGES: CLCTL_CODES = CLCTL_CODES(713i32);
pub const CLCTL_SET_ACCOUNT_ACCESS: CLCTL_CODES = CLCTL_CODES(4194546i32);
pub const CLCTL_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES: CLCTL_CODES = CLCTL_CODES(4205934i32);
pub const CLCTL_SET_CLUSTER_S2D_ENABLED: CLCTL_CODES = CLCTL_CODES(4205922i32);
pub const CLCTL_SET_COMMON_PROPERTIES: CLCTL_CODES = CLCTL_CODES(4194398i32);
pub const CLCTL_SET_CSV_MAINTENANCE_MODE: CLCTL_CODES = CLCTL_CODES(4194966i32);
pub const CLCTL_SET_DNS_DOMAIN: CLCTL_CODES = CLCTL_CODES(4195082i32);
pub const CLCTL_SET_INFRASTRUCTURE_SOFS_BUFFER: CLCTL_CODES = CLCTL_CODES(4205966i32);
pub const CLCTL_SET_MAINTENANCE_MODE: CLCTL_CODES = CLCTL_CODES(4194790i32);
pub const CLCTL_SET_NAME: CLCTL_CODES = CLCTL_CODES(5242918i32);
pub const CLCTL_SET_PRIVATE_PROPERTIES: CLCTL_CODES = CLCTL_CODES(4194438i32);
pub const CLCTL_SET_SHARED_VOLUME_BACKUP_MODE: CLCTL_CODES = CLCTL_CODES(4194970i32);
pub const CLCTL_SET_STORAGE_CONFIGURATION: CLCTL_CODES = CLCTL_CODES(4195042i32);
pub const CLCTL_SHUTDOWN: CLCTL_CODES = CLCTL_CODES(77i32);
pub const CLCTL_STARTING_PHASE1: CLCTL_CODES = CLCTL_CODES(5242934i32);
pub const CLCTL_STARTING_PHASE2: CLCTL_CODES = CLCTL_CODES(5242938i32);
pub const CLCTL_STATE_CHANGE_REASON: CLCTL_CODES = CLCTL_CODES(5242958i32);
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS: CLCTL_CODES = CLCTL_CODES(405i32);
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX: CLCTL_CODES = CLCTL_CODES(501i32);
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX2_INT: CLCTL_CODES = CLCTL_CODES(8161i32);
pub const CLCTL_STORAGE_GET_CLUSBFLT_PATHINFO: CLCTL_CODES = CLCTL_CODES(769i32);
pub const CLCTL_STORAGE_GET_CLUSBFLT_PATHS: CLCTL_CODES = CLCTL_CODES(765i32);
pub const CLCTL_STORAGE_GET_CLUSPORT_DISK_COUNT: CLCTL_CODES = CLCTL_CODES(509i32);
pub const CLCTL_STORAGE_GET_DIRTY: CLCTL_CODES = CLCTL_CODES(537i32);
pub const CLCTL_STORAGE_GET_DISKID: CLCTL_CODES = CLCTL_CODES(517i32);
pub const CLCTL_STORAGE_GET_DISK_INFO: CLCTL_CODES = CLCTL_CODES(401i32);
pub const CLCTL_STORAGE_GET_DISK_INFO_EX: CLCTL_CODES = CLCTL_CODES(497i32);
pub const CLCTL_STORAGE_GET_DISK_INFO_EX2: CLCTL_CODES = CLCTL_CODES(505i32);
pub const CLCTL_STORAGE_GET_DISK_NUMBER_INFO: CLCTL_CODES = CLCTL_CODES(417i32);
pub const CLCTL_STORAGE_GET_DRIVELETTERS: CLCTL_CODES = CLCTL_CODES(493i32);
pub const CLCTL_STORAGE_GET_MOUNTPOINTS: CLCTL_CODES = CLCTL_CODES(529i32);
pub const CLCTL_STORAGE_GET_PHYSICAL_DISK_INFO: CLCTL_CODES = CLCTL_CODES(761i32);
pub const CLCTL_STORAGE_GET_RESOURCEID: CLCTL_CODES = CLCTL_CODES(557i32);
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_INFO: CLCTL_CODES = CLCTL_CODES(549i32);
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES: CLCTL_CODES = CLCTL_CODES(669i32);
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_STATES: CLCTL_CODES = CLCTL_CODES(4194978i32);
pub const CLCTL_STORAGE_IS_CLUSTERABLE: CLCTL_CODES = CLCTL_CODES(521i32);
pub const CLCTL_STORAGE_IS_CSV_FILE: CLCTL_CODES = CLCTL_CODES(553i32);
pub const CLCTL_STORAGE_IS_PATH_VALID: CLCTL_CODES = CLCTL_CODES(409i32);
pub const CLCTL_STORAGE_IS_SHARED_VOLUME: CLCTL_CODES = CLCTL_CODES(677i32);
pub const CLCTL_STORAGE_REMAP_DRIVELETTER: CLCTL_CODES = CLCTL_CODES(513i32);
pub const CLCTL_STORAGE_REMOVE_VM_OWNERSHIP: CLCTL_CODES = CLCTL_CODES(4194830i32);
pub const CLCTL_STORAGE_RENAME_SHARED_VOLUME: CLCTL_CODES = CLCTL_CODES(11734i32);
pub const CLCTL_STORAGE_RENAME_SHARED_VOLUME_GUID: CLCTL_CODES = CLCTL_CODES(11738i32);
pub const CLCTL_STORAGE_SET_DRIVELETTER: CLCTL_CODES = CLCTL_CODES(4194794i32);
pub const CLCTL_STORAGE_SYNC_CLUSDISK_DB: CLCTL_CODES = CLCTL_CODES(4194718i32);
pub const CLCTL_UNDELETE: CLCTL_CODES = CLCTL_CODES(5243014i32);
pub const CLCTL_UNKNOWN: CLCTL_CODES = CLCTL_CODES(0i32);
pub const CLCTL_USER_SHIFT: u32 = 21u32;
pub const CLCTL_VALIDATE_CHANGE_GROUP: CLCTL_CODES = CLCTL_CODES(1057061i32);
pub const CLCTL_VALIDATE_COMMON_PROPERTIES: CLCTL_CODES = CLCTL_CODES(97i32);
pub const CLCTL_VALIDATE_DIRECTORY: CLCTL_CODES = CLCTL_CODES(569i32);
pub const CLCTL_VALIDATE_NETNAME: CLCTL_CODES = CLCTL_CODES(565i32);
pub const CLCTL_VALIDATE_PATH: CLCTL_CODES = CLCTL_CODES(561i32);
pub const CLCTL_VALIDATE_PRIVATE_PROPERTIES: CLCTL_CODES = CLCTL_CODES(137i32);
pub const CLOUD_WITNESS_CONTAINER_NAME: windows_core::PCWSTR = windows_core::w!("msft-cloud-witness");
pub const CLRES_VERSION_V1_00: u32 = 256u32;
pub const CLRES_VERSION_V2_00: u32 = 512u32;
pub const CLRES_VERSION_V3_00: u32 = 768u32;
pub const CLRES_VERSION_V4_00: u32 = 1024u32;
pub const CLUADMEX_OT_CLUSTER: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(1i32);
pub const CLUADMEX_OT_GROUP: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(3i32);
pub const CLUADMEX_OT_NETINTERFACE: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(7i32);
pub const CLUADMEX_OT_NETWORK: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(6i32);
pub const CLUADMEX_OT_NODE: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(2i32);
pub const CLUADMEX_OT_NONE: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(0i32);
pub const CLUADMEX_OT_RESOURCE: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(4i32);
pub const CLUADMEX_OT_RESOURCETYPE: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(5i32);
pub const CLUSAPI_CHANGE_ACCESS: i32 = 2i32;
pub const CLUSAPI_CHANGE_RESOURCE_GROUP_FORCE_MOVE_TO_CSV: u64 = 1u64;
pub const CLUSAPI_GROUP_MOVE_FAILBACK: u32 = 16u32;
pub const CLUSAPI_GROUP_MOVE_HIGH_PRIORITY_START: u32 = 8u32;
pub const CLUSAPI_GROUP_MOVE_IGNORE_AFFINITY_RULE: u32 = 32u32;
pub const CLUSAPI_GROUP_MOVE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUSAPI_GROUP_MOVE_QUEUE_ENABLED: u32 = 4u32;
pub const CLUSAPI_GROUP_MOVE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 2u32;
pub const CLUSAPI_GROUP_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUSAPI_GROUP_ONLINE_BEST_POSSIBLE_NODE: u32 = 4u32;
pub const CLUSAPI_GROUP_ONLINE_IGNORE_AFFINITY_RULE: u32 = 8u32;
pub const CLUSAPI_GROUP_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUSAPI_GROUP_ONLINE_SYNCHRONOUS: u32 = 2u32;
pub const CLUSAPI_NODE_AVOID_PLACEMENT: u32 = 2u32;
pub const CLUSAPI_NODE_PAUSE_REMAIN_ON_PAUSED_NODE_ON_MOVE_ERROR: u32 = 1u32;
pub const CLUSAPI_NODE_PAUSE_RETRY_DRAIN_ON_FAILURE: u32 = 4u32;
pub const CLUSAPI_NODE_RESUME_FAILBACK_PINNED_VMS_ONLY: u32 = 4u32;
pub const CLUSAPI_NODE_RESUME_FAILBACK_STORAGE: u32 = 1u32;
pub const CLUSAPI_NODE_RESUME_FAILBACK_VMS: u32 = 2u32;
pub const CLUSAPI_NO_ACCESS: i32 = 4i32;
pub const CLUSAPI_READ_ACCESS: i32 = 1i32;
pub const CLUSAPI_RESOURCE_OFFLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 4u32;
pub const CLUSAPI_RESOURCE_OFFLINE_FORCE_WITH_TERMINATION: u32 = 2u32;
pub const CLUSAPI_RESOURCE_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_BEING_DELETED: u32 = 8u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_BEING_RESTARTED: u32 = 16u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_MOVING: u32 = 2u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_NONE: u32 = 0u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_PREEMPTED: u32 = 32u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_SHUTTING_DOWN: u32 = 64u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_UNKNOWN: u32 = 1u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_USER_REQUESTED: u32 = 4u32;
pub const CLUSAPI_RESOURCE_ONLINE_BEST_POSSIBLE_NODE: u32 = 8u32;
pub const CLUSAPI_RESOURCE_ONLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 2u32;
pub const CLUSAPI_RESOURCE_ONLINE_IGNORE_AFFINITY_RULE: u32 = 32u32;
pub const CLUSAPI_RESOURCE_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUSAPI_RESOURCE_ONLINE_NECESSARY_FOR_QUORUM: u32 = 4u32;
pub const CLUSAPI_VALID_CHANGE_RESOURCE_GROUP_FLAGS: u64 = 1u64;
pub const CLUSAPI_VERSION: u32 = 2572u32;
pub const CLUSAPI_VERSION_NI: u32 = 2572u32;
pub const CLUSAPI_VERSION_RS3: u32 = 2560u32;
pub const CLUSAPI_VERSION_SERVER2008: u32 = 1536u32;
pub const CLUSAPI_VERSION_SERVER2008R2: u32 = 1792u32;
pub const CLUSAPI_VERSION_WINDOWS8: u32 = 1793u32;
pub const CLUSAPI_VERSION_WINDOWSBLUE: u32 = 1794u32;
pub const CLUSAPI_VERSION_WINTHRESHOLD: u32 = 1795u32;
pub const CLUSCTL_ACCESS_MODE_MASK: u32 = 3u32;
pub const CLUSCTL_ACCESS_SHIFT: u32 = 0u32;
pub const CLUSCTL_AFFINITYRULE_GET_COMMON_PROPERTIES: CLUSCTL_AFFINITYRULE_CODES = CLUSCTL_AFFINITYRULE_CODES(150995033i32);
pub const CLUSCTL_AFFINITYRULE_GET_GROUPNAMES: CLUSCTL_AFFINITYRULE_CODES = CLUSCTL_AFFINITYRULE_CODES(151006577i32);
pub const CLUSCTL_AFFINITYRULE_GET_ID: CLUSCTL_AFFINITYRULE_CODES = CLUSCTL_AFFINITYRULE_CODES(150995001i32);
pub const CLUSCTL_AFFINITYRULE_GET_RO_COMMON_PROPERTIES: CLUSCTL_AFFINITYRULE_CODES = CLUSCTL_AFFINITYRULE_CODES(150995029i32);
pub const CLUSCTL_AFFINITYRULE_SET_COMMON_PROPERTIES: CLUSCTL_AFFINITYRULE_CODES = CLUSCTL_AFFINITYRULE_CODES(155189342i32);
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562849i32);
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562865i32);
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20979958i32);
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20979942i32);
pub const CLUSCTL_CLUSTER_BATCH_BLOCK_KEY: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441086i32);
pub const CLUSCTL_CLUSTER_BATCH_UNBLOCK_KEY: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441089i32);
pub const CLUSCTL_CLUSTER_CHECK_VOTER_DOWN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440585i32);
pub const CLUSCTL_CLUSTER_CHECK_VOTER_DOWN_WITNESS: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440625i32);
pub const CLUSCTL_CLUSTER_CHECK_VOTER_EVICT: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440581i32);
pub const CLUSCTL_CLUSTER_CHECK_VOTER_EVICT_WITNESS: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440621i32);
pub const CLUSCTL_CLUSTER_CLEAR_NODE_CONNECTION_INFO: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121635590i32);
pub const CLUSCTL_CLUSTER_ENUM_AFFINITY_RULE_NAMES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117452253i32);
pub const CLUSCTL_CLUSTER_ENUM_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440593i32);
pub const CLUSCTL_CLUSTER_ENUM_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440633i32);
pub const CLUSCTL_CLUSTER_FORCE_FLUSH_DB: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121646566i32);
pub const CLUSCTL_CLUSTER_GET_CLMUSR_TOKEN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440877i32);
pub const CLUSCTL_CLUSTER_GET_CLUSDB_TIMESTAMP: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441193i32);
pub const CLUSCTL_CLUSTER_GET_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440601i32);
pub const CLUSCTL_CLUSTER_GET_COMMON_PROPERTY_FMTS: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440613i32);
pub const CLUSCTL_CLUSTER_GET_FQDN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440573i32);
pub const CLUSCTL_CLUSTER_GET_GUM_LOCK_OWNER: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441209i32);
pub const CLUSCTL_CLUSTER_GET_NODES_IN_FD: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117452257i32);
pub const CLUSCTL_CLUSTER_GET_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440641i32);
pub const CLUSCTL_CLUSTER_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440653i32);
pub const CLUSCTL_CLUSTER_GET_RO_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440597i32);
pub const CLUSCTL_CLUSTER_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440637i32);
pub const CLUSCTL_CLUSTER_GET_SHARED_VOLUME_ID: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441169i32);
pub const CLUSCTL_CLUSTER_GET_STORAGE_CONFIGURATION: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441253i32);
pub const CLUSCTL_CLUSTER_GET_STORAGE_CONFIG_ATTRIBUTES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441257i32);
pub const CLUSCTL_CLUSTER_RELOAD_AUTOLOGGER_CONFIG: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117452242i32);
pub const CLUSCTL_CLUSTER_REMOVE_NODE: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121635566i32);
pub const CLUSCTL_CLUSTER_SET_ACCOUNT_ACCESS: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121635058i32);
pub const CLUSCTL_CLUSTER_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121646446i32);
pub const CLUSCTL_CLUSTER_SET_CLUSTER_S2D_ENABLED: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121646434i32);
pub const CLUSCTL_CLUSTER_SET_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121634910i32);
pub const CLUSCTL_CLUSTER_SET_DNS_DOMAIN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121635594i32);
pub const CLUSCTL_CLUSTER_SET_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121634950i32);
pub const CLUSCTL_CLUSTER_SET_STORAGE_CONFIGURATION: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121635554i32);
pub const CLUSCTL_CLUSTER_SHUTDOWN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440589i32);
pub const CLUSCTL_CLUSTER_STORAGE_RENAME_SHARED_VOLUME: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117452246i32);
pub const CLUSCTL_CLUSTER_STORAGE_RENAME_SHARED_VOLUME_GUID: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117452250i32);
pub const CLUSCTL_CLUSTER_UNKNOWN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440512i32);
pub const CLUSCTL_CLUSTER_VALIDATE_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440609i32);
pub const CLUSCTL_CLUSTER_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440649i32);
pub const CLUSCTL_CONTROL_CODE_MASK: u32 = 4194303u32;
pub const CLUSCTL_FUNCTION_SHIFT: u32 = 2u32;
pub const CLUSCTL_GET_OPERATION_CONTEXT_PARAMS_VERSION_1: u32 = 1u32;
pub const CLUSCTL_GROUPSET_GET_COMMON_PROPERTIES: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134217817i32);
pub const CLUSCTL_GROUPSET_GET_GROUPS: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134229361i32);
pub const CLUSCTL_GROUPSET_GET_ID: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134217785i32);
pub const CLUSCTL_GROUPSET_GET_PROVIDER_GROUPS: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134229365i32);
pub const CLUSCTL_GROUPSET_GET_PROVIDER_GROUPSETS: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134229369i32);
pub const CLUSCTL_GROUPSET_GET_RO_COMMON_PROPERTIES: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134217813i32);
pub const CLUSCTL_GROUPSET_SET_COMMON_PROPERTIES: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(138412126i32);
pub const CLUSCTL_GROUP_ENUM_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331729i32);
pub const CLUSCTL_GROUP_ENUM_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331769i32);
pub const CLUSCTL_GROUP_GET_CHARACTERISTICS: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331653i32);
pub const CLUSCTL_GROUP_GET_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331737i32);
pub const CLUSCTL_GROUP_GET_COMMON_PROPERTY_FMTS: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331749i32);
pub const CLUSCTL_GROUP_GET_FAILURE_INFO: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331673i32);
pub const CLUSCTL_GROUP_GET_FLAGS: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331657i32);
pub const CLUSCTL_GROUP_GET_ID: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331705i32);
pub const CLUSCTL_GROUP_GET_LAST_MOVE_TIME: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50332377i32);
pub const CLUSCTL_GROUP_GET_NAME: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331689i32);
pub const CLUSCTL_GROUP_GET_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331777i32);
pub const CLUSCTL_GROUP_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331789i32);
pub const CLUSCTL_GROUP_GET_PROVIDER_GROUPS: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134229373i32);
pub const CLUSCTL_GROUP_GET_PROVIDER_GROUPSETS: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134229377i32);
pub const CLUSCTL_GROUP_GET_RO_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331733i32);
pub const CLUSCTL_GROUP_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331773i32);
pub const CLUSCTL_GROUP_QUERY_DELETE: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50332089i32);
pub const CLUSCTL_GROUP_SET_CCF_FROM_MASTER: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(54537606i32);
pub const CLUSCTL_GROUP_SET_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(54526046i32);
pub const CLUSCTL_GROUP_SET_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(54526086i32);
pub const CLUSCTL_GROUP_UNKNOWN: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331648i32);
pub const CLUSCTL_GROUP_VALIDATE_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331745i32);
pub const CLUSCTL_GROUP_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331785i32);
pub const CLUSCTL_NETINTERFACE_ENUM_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663377i32);
pub const CLUSCTL_NETINTERFACE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663417i32);
pub const CLUSCTL_NETINTERFACE_GET_CHARACTERISTICS: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663301i32);
pub const CLUSCTL_NETINTERFACE_GET_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663385i32);
pub const CLUSCTL_NETINTERFACE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663397i32);
pub const CLUSCTL_NETINTERFACE_GET_FLAGS: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663305i32);
pub const CLUSCTL_NETINTERFACE_GET_ID: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663353i32);
pub const CLUSCTL_NETINTERFACE_GET_NAME: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663337i32);
pub const CLUSCTL_NETINTERFACE_GET_NETWORK: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663349i32);
pub const CLUSCTL_NETINTERFACE_GET_NODE: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663345i32);
pub const CLUSCTL_NETINTERFACE_GET_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663425i32);
pub const CLUSCTL_NETINTERFACE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663437i32);
pub const CLUSCTL_NETINTERFACE_GET_RO_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663381i32);
pub const CLUSCTL_NETINTERFACE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663421i32);
pub const CLUSCTL_NETINTERFACE_SET_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(104857694i32);
pub const CLUSCTL_NETINTERFACE_SET_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(104857734i32);
pub const CLUSCTL_NETINTERFACE_UNKNOWN: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663296i32);
pub const CLUSCTL_NETINTERFACE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663393i32);
pub const CLUSCTL_NETINTERFACE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663433i32);
pub const CLUSCTL_NETWORK_ENUM_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886161i32);
pub const CLUSCTL_NETWORK_ENUM_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886201i32);
pub const CLUSCTL_NETWORK_GET_CHARACTERISTICS: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886085i32);
pub const CLUSCTL_NETWORK_GET_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886169i32);
pub const CLUSCTL_NETWORK_GET_COMMON_PROPERTY_FMTS: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886181i32);
pub const CLUSCTL_NETWORK_GET_FLAGS: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886089i32);
pub const CLUSCTL_NETWORK_GET_ID: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886137i32);
pub const CLUSCTL_NETWORK_GET_NAME: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886121i32);
pub const CLUSCTL_NETWORK_GET_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886209i32);
pub const CLUSCTL_NETWORK_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886221i32);
pub const CLUSCTL_NETWORK_GET_RO_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886165i32);
pub const CLUSCTL_NETWORK_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886205i32);
pub const CLUSCTL_NETWORK_SET_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(88080478i32);
pub const CLUSCTL_NETWORK_SET_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(88080518i32);
pub const CLUSCTL_NETWORK_UNKNOWN: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886080i32);
pub const CLUSCTL_NETWORK_VALIDATE_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886177i32);
pub const CLUSCTL_NETWORK_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886217i32);
pub const CLUSCTL_NODE_BLOCK_GEM_SEND_RECV: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109581i32);
pub const CLUSCTL_NODE_ENUM_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108945i32);
pub const CLUSCTL_NODE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108985i32);
pub const CLUSCTL_NODE_GET_CHARACTERISTICS: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108869i32);
pub const CLUSCTL_NODE_GET_CLUSTER_SERVICE_ACCOUNT_NAME: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108929i32);
pub const CLUSCTL_NODE_GET_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108953i32);
pub const CLUSCTL_NODE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108965i32);
pub const CLUSCTL_NODE_GET_FLAGS: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108873i32);
pub const CLUSCTL_NODE_GET_GEMID_VECTOR: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109585i32);
pub const CLUSCTL_NODE_GET_ID: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108921i32);
pub const CLUSCTL_NODE_GET_NAME: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108905i32);
pub const CLUSCTL_NODE_GET_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108993i32);
pub const CLUSCTL_NODE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109005i32);
pub const CLUSCTL_NODE_GET_RO_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108949i32);
pub const CLUSCTL_NODE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108989i32);
pub const CLUSCTL_NODE_GET_STUCK_NODES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109565i32);
pub const CLUSCTL_NODE_INJECT_GEM_FAULT: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109569i32);
pub const CLUSCTL_NODE_INTRODUCE_GEM_REPAIR_DELAY: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109573i32);
pub const CLUSCTL_NODE_SEND_DUMMY_GEM_MESSAGES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109577i32);
pub const CLUSCTL_NODE_SET_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(71303262i32);
pub const CLUSCTL_NODE_SET_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(71303302i32);
pub const CLUSCTL_NODE_UNKNOWN: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108864i32);
pub const CLUSCTL_NODE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108961i32);
pub const CLUSCTL_NODE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109001i32);
pub const CLUSCTL_OBJECT_MASK: u32 = 255u32;
pub const CLUSCTL_OBJECT_SHIFT: u32 = 24u32;
pub const CLUSCTL_RESOURCE_ADD_CRYPTO_CHECKPOINT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971694i32);
pub const CLUSCTL_RESOURCE_ADD_CRYPTO_CHECKPOINT_EX: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972246i32);
pub const CLUSCTL_RESOURCE_ADD_DEPENDENCY: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020114i32);
pub const CLUSCTL_RESOURCE_ADD_OWNER: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020122i32);
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971682i32);
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT_32BIT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971714i32);
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT_64BIT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971710i32);
pub const CLUSCTL_RESOURCE_CHECK_DRAIN_VETO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(17834285i32);
pub const CLUSCTL_RESOURCE_CLUSTER_NAME_CHANGED: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020138i32);
pub const CLUSCTL_RESOURCE_CLUSTER_VERSION_CHANGED: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020142i32);
pub const CLUSCTL_RESOURCE_DELETE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020102i32);
pub const CLUSCTL_RESOURCE_DELETE_CRYPTO_CHECKPOINT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971698i32);
pub const CLUSCTL_RESOURCE_DELETE_REGISTRY_CHECKPOINT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971686i32);
pub const CLUSCTL_RESOURCE_DISABLE_SHARED_VOLUME_DIRECTIO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972174i32);
pub const CLUSCTL_RESOURCE_ENABLE_SHARED_VOLUME_DIRECTIO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972170i32);
pub const CLUSCTL_RESOURCE_ENUM_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777297i32);
pub const CLUSCTL_RESOURCE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777337i32);
pub const CLUSCTL_RESOURCE_EVICT_NODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020110i32);
pub const CLUSCTL_RESOURCE_FORCE_QUORUM: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020166i32);
pub const CLUSCTL_RESOURCE_FSWITNESS_GET_EPOCH_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(17825885i32);
pub const CLUSCTL_RESOURCE_FSWITNESS_RELEASE_LOCK: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020198i32);
pub const CLUSCTL_RESOURCE_FSWITNESS_SET_EPOCH_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020194i32);
pub const CLUSCTL_RESOURCE_GET_CHARACTERISTICS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777221i32);
pub const CLUSCTL_RESOURCE_GET_CLASS_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777229i32);
pub const CLUSCTL_RESOURCE_GET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777305i32);
pub const CLUSCTL_RESOURCE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777317i32);
pub const CLUSCTL_RESOURCE_GET_CRYPTO_CHECKPOINTS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777397i32);
pub const CLUSCTL_RESOURCE_GET_DNS_NAME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777589i32);
pub const CLUSCTL_RESOURCE_GET_FAILURE_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777241i32);
pub const CLUSCTL_RESOURCE_GET_FLAGS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777225i32);
pub const CLUSCTL_RESOURCE_GET_ID: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777273i32);
pub const CLUSCTL_RESOURCE_GET_INFRASTRUCTURE_SOFS_BUFFER: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16788873i32);
pub const CLUSCTL_RESOURCE_GET_LOADBAL_PROCESS_LIST: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777417i32);
pub const CLUSCTL_RESOURCE_GET_NAME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777257i32);
pub const CLUSCTL_RESOURCE_GET_NETWORK_NAME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777577i32);
pub const CLUSCTL_RESOURCE_GET_NODES_IN_FD: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16788961i32);
pub const CLUSCTL_RESOURCE_GET_OPERATION_CONTEXT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(17834217i32);
pub const CLUSCTL_RESOURCE_GET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777345i32);
pub const CLUSCTL_RESOURCE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777357i32);
pub const CLUSCTL_RESOURCE_GET_REGISTRY_CHECKPOINTS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777385i32);
pub const CLUSCTL_RESOURCE_GET_REQUIRED_DEPENDENCIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777233i32);
pub const CLUSCTL_RESOURCE_GET_RESOURCE_TYPE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777261i32);
pub const CLUSCTL_RESOURCE_GET_RO_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777301i32);
pub const CLUSCTL_RESOURCE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777341i32);
pub const CLUSCTL_RESOURCE_GET_STATE_CHANGE_TIME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16788829i32);
pub const CLUSCTL_RESOURCE_INITIALIZE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020170i32);
pub const CLUSCTL_RESOURCE_INSTALL_NODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020106i32);
pub const CLUSCTL_RESOURCE_IPADDRESS_RELEASE_LEASE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971970i32);
pub const CLUSCTL_RESOURCE_IPADDRESS_RENEW_LEASE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971966i32);
pub const CLUSCTL_RESOURCE_IS_QUORUM_BLOCKED: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777905i32);
pub const CLUSCTL_RESOURCE_JOINING_GROUP: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020186i32);
pub const CLUSCTL_RESOURCE_LEAVING_GROUP: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020182i32);
pub const CLUSCTL_RESOURCE_NETNAME_CREDS_NOTIFYCAM: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020202i32);
pub const CLUSCTL_RESOURCE_NETNAME_DELETE_CO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777598i32);
pub const CLUSCTL_RESOURCE_NETNAME_GET_VIRTUAL_SERVER_TOKEN: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777581i32);
pub const CLUSCTL_RESOURCE_NETNAME_REGISTER_DNS_RECORDS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777586i32);
pub const CLUSCTL_RESOURCE_NETNAME_REPAIR_VCO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777613i32);
pub const CLUSCTL_RESOURCE_NETNAME_RESET_VCO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777605i32);
pub const CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777594i32);
pub const CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFOEX: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16778010i32);
pub const CLUSCTL_RESOURCE_NETNAME_VALIDATE_VCO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777601i32);
pub const CLUSCTL_RESOURCE_NOTIFY_DRAIN_COMPLETE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(17834289i32);
pub const CLUSCTL_RESOURCE_NOTIFY_OWNER_CHANGE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22028578i32);
pub const CLUSCTL_RESOURCE_NOTIFY_QUORUM_STATUS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020222i32);
pub const CLUSCTL_RESOURCE_POOL_GET_DRIVE_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777909i32);
pub const CLUSCTL_RESOURCE_PREPARE_UPGRADE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20979946i32);
pub const CLUSCTL_RESOURCE_PROVIDER_STATE_CHANGE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020178i32);
pub const CLUSCTL_RESOURCE_QUERY_DELETE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777657i32);
pub const CLUSCTL_RESOURCE_QUERY_MAINTENANCE_MODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777697i32);
pub const CLUSCTL_RESOURCE_REMOVE_DEPENDENCY: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020118i32);
pub const CLUSCTL_RESOURCE_REMOVE_OWNER: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020126i32);
pub const CLUSCTL_RESOURCE_RLUA_GET_VIRTUAL_SERVER_TOKEN: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777581i32);
pub const CLUSCTL_RESOURCE_RLUA_SET_PWD_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777594i32);
pub const CLUSCTL_RESOURCE_RLUA_SET_PWD_INFOEX: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16778010i32);
pub const CLUSCTL_RESOURCE_RW_MODIFY_NOOP: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972206i32);
pub const CLUSCTL_RESOURCE_SCALEOUT_COMMAND: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20983190i32);
pub const CLUSCTL_RESOURCE_SCALEOUT_CONTROL: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20983194i32);
pub const CLUSCTL_RESOURCE_SCALEOUT_GET_CLUSTERS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20983197i32);
pub const CLUSCTL_RESOURCE_SET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971614i32);
pub const CLUSCTL_RESOURCE_SET_CSV_MAINTENANCE_MODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972182i32);
pub const CLUSCTL_RESOURCE_SET_INFRASTRUCTURE_SOFS_BUFFER: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20983182i32);
pub const CLUSCTL_RESOURCE_SET_MAINTENANCE_MODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972006i32);
pub const CLUSCTL_RESOURCE_SET_NAME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020134i32);
pub const CLUSCTL_RESOURCE_SET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971654i32);
pub const CLUSCTL_RESOURCE_SET_SHARED_VOLUME_BACKUP_MODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972186i32);
pub const CLUSCTL_RESOURCE_STATE_CHANGE_REASON: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020174i32);
pub const CLUSCTL_RESOURCE_STATE_CHANGE_REASON_VERSION_1: u32 = 1u32;
pub const CLUSCTL_RESOURCE_STORAGE_GET_DIRTY: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777753i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISKID: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777733i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777617i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777713i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX2: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777721i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_NUMBER_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777633i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_MOUNTPOINTS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777745i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777765i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777885i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_STATES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972194i32);
pub const CLUSCTL_RESOURCE_STORAGE_IS_PATH_VALID: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777625i32);
pub const CLUSCTL_RESOURCE_STORAGE_IS_SHARED_VOLUME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777893i32);
pub const CLUSCTL_RESOURCE_STORAGE_RENAME_SHARED_VOLUME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16788950i32);
pub const CLUSCTL_RESOURCE_STORAGE_RENAME_SHARED_VOLUME_GUID: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16788954i32);
pub const CLUSCTL_RESOURCE_STORAGE_SET_DRIVELETTER: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972010i32);
pub const CLUSCTL_RESOURCE_TYPE_CHECK_DRAIN_VETO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(34611501i32);
pub const CLUSCTL_RESOURCE_TYPE_CLUSTER_VERSION_CHANGED: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797358i32);
pub const CLUSCTL_RESOURCE_TYPE_ENUM_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554513i32);
pub const CLUSCTL_RESOURCE_TYPE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554553i32);
pub const CLUSCTL_RESOURCE_TYPE_EVICT_NODE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797326i32);
pub const CLUSCTL_RESOURCE_TYPE_FIXUP_ON_UPGRADE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797362i32);
pub const CLUSCTL_RESOURCE_TYPE_GEN_APP_VALIDATE_DIRECTORY: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33555001i32);
pub const CLUSCTL_RESOURCE_TYPE_GEN_APP_VALIDATE_PATH: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554993i32);
pub const CLUSCTL_RESOURCE_TYPE_GEN_SCRIPT_VALIDATE_PATH: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554993i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_ARB_TIMEOUT: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554453i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_CHARACTERISTICS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554437i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_CLASS_INFO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554445i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554521i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554533i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_RESOURCE_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554537i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_CRYPTO_CHECKPOINTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554613i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_FLAGS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554441i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554561i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554573i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_RESOURCE_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554577i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_REGISTRY_CHECKPOINTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554601i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_REQUIRED_DEPENDENCIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554449i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_RO_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554517i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554557i32);
pub const CLUSCTL_RESOURCE_TYPE_HOLD_IO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797374i32);
pub const CLUSCTL_RESOURCE_TYPE_INSTALL_NODE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797322i32);
pub const CLUSCTL_RESOURCE_TYPE_NETNAME_GET_OU_FOR_VCO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37749358i32);
pub const CLUSCTL_RESOURCE_TYPE_NETNAME_VALIDATE_NETNAME: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554997i32);
pub const CLUSCTL_RESOURCE_TYPE_NOTIFY_DRAIN_COMPLETE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(34611505i32);
pub const CLUSCTL_RESOURCE_TYPE_NOTIFY_MONITOR_SHUTTING_DOWN: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(34603137i32);
pub const CLUSCTL_RESOURCE_TYPE_PREPARE_UPGRADE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37757162i32);
pub const CLUSCTL_RESOURCE_TYPE_QUERY_DELETE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554873i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_ADD_REPLICATION_GROUP: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562946i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_LOGDISKS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562953i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562961i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562957i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_LOG_INFO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562949i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_LOG_VOLUME: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562973i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICATED_DISKS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562965i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICATED_PARTITION_INFO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562981i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICA_VOLUMES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562969i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_RESOURCE_GROUP: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562977i32);
pub const CLUSCTL_RESOURCE_TYPE_RESUME_IO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797378i32);
pub const CLUSCTL_RESOURCE_TYPE_SET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37748830i32);
pub const CLUSCTL_RESOURCE_TYPE_SET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37748870i32);
pub const CLUSCTL_RESOURCE_TYPE_STARTING_PHASE1: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797366i32);
pub const CLUSCTL_RESOURCE_TYPE_STARTING_PHASE2: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797370i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554837i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554933i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_ADD_VOLUME_INFO: u32 = 1u32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_FILTER_BY_POOL: u32 = 2u32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_INCLUDE_NON_SHARED_DISKS: u32 = 4u32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INT: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562593i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_DISKID: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554949i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_DRIVELETTERS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554925i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_RESOURCEID: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554989i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_IS_CLUSTERABLE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554953i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_IS_CSV_FILE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(16777769i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_REMAP_DRIVELETTER: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554945i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_REMOVE_VM_OWNERSHIP: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37749262i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_SYNC_CLUSDISK_DB: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37749150i32);
pub const CLUSCTL_RESOURCE_TYPE_UNKNOWN: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554432i32);
pub const CLUSCTL_RESOURCE_TYPE_UPGRADE_COMPLETED: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37757166i32);
pub const CLUSCTL_RESOURCE_TYPE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554529i32);
pub const CLUSCTL_RESOURCE_TYPE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554569i32);
pub const CLUSCTL_RESOURCE_TYPE_WITNESS_VALIDATE_PATH: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554993i32);
pub const CLUSCTL_RESOURCE_UNDELETE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020230i32);
pub const CLUSCTL_RESOURCE_UNKNOWN: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777216i32);
pub const CLUSCTL_RESOURCE_UPGRADE_COMPLETED: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20979950i32);
pub const CLUSCTL_RESOURCE_UPGRADE_DLL: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971706i32);
pub const CLUSCTL_RESOURCE_VALIDATE_CHANGE_GROUP: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(17834277i32);
pub const CLUSCTL_RESOURCE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777313i32);
pub const CLUSCTL_RESOURCE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777353i32);
pub const CLUSGROUPSET_STATUS_APPLICATION_READY: u64 = 8u64;
pub const CLUSGROUPSET_STATUS_GROUPS_ONLINE: u64 = 2u64;
pub const CLUSGROUPSET_STATUS_GROUPS_PENDING: u64 = 1u64;
pub const CLUSGROUPSET_STATUS_OS_HEARTBEAT: u64 = 4u64;
pub const CLUSGRP_STATUS_APPLICATION_READY: u64 = 1024u64;
pub const CLUSGRP_STATUS_EMBEDDED_FAILURE: u64 = 32u64;
pub const CLUSGRP_STATUS_LOCKED_MODE: u64 = 1u64;
pub const CLUSGRP_STATUS_NETWORK_FAILURE: u64 = 128u64;
pub const CLUSGRP_STATUS_OFFLINE_DUE_TO_ANTIAFFINITY_CONFLICT: u64 = 64u64;
pub const CLUSGRP_STATUS_OFFLINE_NOT_LOCAL_DISK_OWNER: u64 = 2048u64;
pub const CLUSGRP_STATUS_OS_HEARTBEAT: u64 = 512u64;
pub const CLUSGRP_STATUS_PHYSICAL_RESOURCES_LACKING: u64 = 8u64;
pub const CLUSGRP_STATUS_PREEMPTED: u64 = 2u64;
pub const CLUSGRP_STATUS_UNMONITORED: u64 = 256u64;
pub const CLUSGRP_STATUS_WAITING_FOR_DEPENDENCIES: u64 = 4096u64;
pub const CLUSGRP_STATUS_WAITING_IN_QUEUE_FOR_MOVE: u64 = 4u64;
pub const CLUSGRP_STATUS_WAITING_TO_START: u64 = 16u64;
pub const CLUSPROP_FORMAT_BINARY: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(1i32);
pub const CLUSPROP_FORMAT_DWORD: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(2i32);
pub const CLUSPROP_FORMAT_EXPANDED_SZ: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(8i32);
pub const CLUSPROP_FORMAT_EXPAND_SZ: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(4i32);
pub const CLUSPROP_FORMAT_FILETIME: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(12i32);
pub const CLUSPROP_FORMAT_LARGE_INTEGER: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(10i32);
pub const CLUSPROP_FORMAT_LONG: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(7i32);
pub const CLUSPROP_FORMAT_MULTI_SZ: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(5i32);
pub const CLUSPROP_FORMAT_PROPERTY_LIST: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(14i32);
pub const CLUSPROP_FORMAT_SECURITY_DESCRIPTOR: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(9i32);
pub const CLUSPROP_FORMAT_SZ: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(3i32);
pub const CLUSPROP_FORMAT_ULARGE_INTEGER: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(6i32);
pub const CLUSPROP_FORMAT_UNKNOWN: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(0i32);
pub const CLUSPROP_FORMAT_USER: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(32768i32);
pub const CLUSPROP_FORMAT_VALUE_LIST: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(13i32);
pub const CLUSPROP_FORMAT_WORD: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(11i32);
pub const CLUSPROP_IPADDR_ENABLENETBIOS_DISABLED: CLUSPROP_IPADDR_ENABLENETBIOS = CLUSPROP_IPADDR_ENABLENETBIOS(0i32);
pub const CLUSPROP_IPADDR_ENABLENETBIOS_ENABLED: CLUSPROP_IPADDR_ENABLENETBIOS = CLUSPROP_IPADDR_ENABLENETBIOS(1i32);
pub const CLUSPROP_IPADDR_ENABLENETBIOS_TRACK_NIC: CLUSPROP_IPADDR_ENABLENETBIOS = CLUSPROP_IPADDR_ENABLENETBIOS(2i32);
pub const CLUSPROP_PIFLAG_DEFAULT_QUORUM: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(8i32);
pub const CLUSPROP_PIFLAG_ENCRYPTION_ENABLED: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(32i32);
pub const CLUSPROP_PIFLAG_RAW: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(64i32);
pub const CLUSPROP_PIFLAG_REMOVABLE: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(2i32);
pub const CLUSPROP_PIFLAG_STICKY: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(1i32);
pub const CLUSPROP_PIFLAG_UNKNOWN: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(-2147483648i32);
pub const CLUSPROP_PIFLAG_USABLE: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(4i32);
pub const CLUSPROP_PIFLAG_USABLE_FOR_CSV: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(16i32);
pub const CLUSPROP_SYNTAX_DISK_GUID: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(720899u32);
pub const CLUSPROP_SYNTAX_DISK_NUMBER: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(458754u32);
pub const CLUSPROP_SYNTAX_DISK_SERIALNUMBER: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(655363u32);
pub const CLUSPROP_SYNTAX_DISK_SIGNATURE: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(327682u32);
pub const CLUSPROP_SYNTAX_DISK_SIZE: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(786438u32);
pub const CLUSPROP_SYNTAX_ENDMARK: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(0u32);
pub const CLUSPROP_SYNTAX_FTSET_INFO: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(589825u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_BINARY: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65537u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_DWORD: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65538u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_EXPANDED_SZ: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65544u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_EXPAND_SZ: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65540u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_FILETIME: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65548u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_LARGE_INTEGER: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65546u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_LONG: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65543u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_MULTI_SZ: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65541u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_PROPERTY_LIST: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65550u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_SECURITY_DESCRIPTOR: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65545u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_SZ: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65539u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_ULARGE_INTEGER: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65542u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_WORD: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65547u32);
pub const CLUSPROP_SYNTAX_NAME: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(262147u32);
pub const CLUSPROP_SYNTAX_PARTITION_INFO: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(524289u32);
pub const CLUSPROP_SYNTAX_PARTITION_INFO_EX: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(851969u32);
pub const CLUSPROP_SYNTAX_PARTITION_INFO_EX2: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(917505u32);
pub const CLUSPROP_SYNTAX_RESCLASS: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(131074u32);
pub const CLUSPROP_SYNTAX_SCSI_ADDRESS: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(393218u32);
pub const CLUSPROP_SYNTAX_STORAGE_DEVICE_ID_DESCRIPTOR: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(983041u32);
pub const CLUSPROP_TYPE_DISK_GUID: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(11i32);
pub const CLUSPROP_TYPE_DISK_NUMBER: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(7i32);
pub const CLUSPROP_TYPE_DISK_SERIALNUMBER: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(10i32);
pub const CLUSPROP_TYPE_DISK_SIZE: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(12i32);
pub const CLUSPROP_TYPE_ENDMARK: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(0i32);
pub const CLUSPROP_TYPE_FTSET_INFO: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(9i32);
pub const CLUSPROP_TYPE_LIST_VALUE: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(1i32);
pub const CLUSPROP_TYPE_NAME: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(4i32);
pub const CLUSPROP_TYPE_PARTITION_INFO: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(8i32);
pub const CLUSPROP_TYPE_PARTITION_INFO_EX: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(13i32);
pub const CLUSPROP_TYPE_PARTITION_INFO_EX2: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(14i32);
pub const CLUSPROP_TYPE_RESCLASS: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(2i32);
pub const CLUSPROP_TYPE_RESERVED1: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(3i32);
pub const CLUSPROP_TYPE_SCSI_ADDRESS: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(6i32);
pub const CLUSPROP_TYPE_SIGNATURE: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(5i32);
pub const CLUSPROP_TYPE_STORAGE_DEVICE_ID_DESCRIPTOR: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(15i32);
pub const CLUSPROP_TYPE_UNKNOWN: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(-1i32);
pub const CLUSPROP_TYPE_USER: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(32768i32);
pub const CLUSREG_COMMAND_NONE: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(0i32);
pub const CLUSREG_CONDITION_EXISTS: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(11i32);
pub const CLUSREG_CONDITION_IS_EQUAL: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(13i32);
pub const CLUSREG_CONDITION_IS_GREATER_THAN: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(15i32);
pub const CLUSREG_CONDITION_IS_LESS_THAN: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(16i32);
pub const CLUSREG_CONDITION_IS_NOT_EQUAL: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(14i32);
pub const CLUSREG_CONDITION_KEY_EXISTS: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(17i32);
pub const CLUSREG_CONDITION_KEY_NOT_EXISTS: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(18i32);
pub const CLUSREG_CONDITION_NOT_EXISTS: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(12i32);
pub const CLUSREG_CONTROL_COMMAND: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(10i32);
pub const CLUSREG_CREATE_KEY: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(2i32);
pub const CLUSREG_DATABASE_ISOLATE_READ: u32 = 2u32;
pub const CLUSREG_DATABASE_SYNC_WRITE_TO_ALL_NODES: u32 = 1u32;
pub const CLUSREG_DELETE_KEY: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(3i32);
pub const CLUSREG_DELETE_VALUE: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(4i32);
pub const CLUSREG_KEYNAME_OBJECTGUIDS: windows_core::PCWSTR = windows_core::w!("ObjectGUIDs");
pub const CLUSREG_LAST_COMMAND: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(19i32);
pub const CLUSREG_NAME_AFFINITYRULE_ENABLED: windows_core::PCWSTR = windows_core::w!("Enabled");
pub const CLUSREG_NAME_AFFINITYRULE_GROUPS: windows_core::PCWSTR = windows_core::w!("Groups");
pub const CLUSREG_NAME_AFFINITYRULE_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_AFFINITYRULE_TYPE: windows_core::PCWSTR = windows_core::w!("RuleType");
pub const CLUSREG_NAME_CLOUDWITNESS_ACCOUNT_NAME: windows_core::PCWSTR = windows_core::w!("AccountName");
pub const CLUSREG_NAME_CLOUDWITNESS_CONTAINER_NAME: windows_core::PCWSTR = windows_core::w!("ContainerName");
pub const CLUSREG_NAME_CLOUDWITNESS_ENDPOINT_INFO: windows_core::PCWSTR = windows_core::w!("EndpointInfo");
pub const CLUSREG_NAME_CLOUDWITNESS_PRIMARY_KEY: windows_core::PCWSTR = windows_core::w!("PrimaryKey");
pub const CLUSREG_NAME_CLOUDWITNESS_PRIMARY_TOKEN: windows_core::PCWSTR = windows_core::w!("PrimaryToken");
pub const CLUSREG_NAME_CLUS_DEFAULT_NETWORK_ROLE: windows_core::PCWSTR = windows_core::w!("DefaultNetworkRole");
pub const CLUSREG_NAME_CLUS_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_CLUS_SD: windows_core::PCWSTR = windows_core::w!("Security Descriptor");
pub const CLUSREG_NAME_CROSS_SITE_DELAY: windows_core::PCWSTR = windows_core::w!("CrossSiteDelay");
pub const CLUSREG_NAME_CROSS_SITE_THRESHOLD: windows_core::PCWSTR = windows_core::w!("CrossSiteThreshold");
pub const CLUSREG_NAME_CROSS_SUBNET_DELAY: windows_core::PCWSTR = windows_core::w!("CrossSubnetDelay");
pub const CLUSREG_NAME_CROSS_SUBNET_THRESHOLD: windows_core::PCWSTR = windows_core::w!("CrossSubnetThreshold");
pub const CLUSREG_NAME_CSV_BLOCK_CACHE: windows_core::PCWSTR = windows_core::w!("BlockCacheSize");
pub const CLUSREG_NAME_CSV_MDS_SD: windows_core::PCWSTR = windows_core::w!("SharedVolumeSecurityDescriptor");
pub const CLUSREG_NAME_DATABASE_READ_WRITE_MODE: windows_core::PCWSTR = windows_core::w!("DatabaseReadWriteMode");
pub const CLUSREG_NAME_DDA_DEVICE_ALLOCATIONS: windows_core::PCWSTR = windows_core::w!("DdaDeviceAllocations");
pub const CLUSREG_NAME_DHCP_BACKUP_PATH: windows_core::PCWSTR = windows_core::w!("BackupPath");
pub const CLUSREG_NAME_DHCP_DATABASE_PATH: windows_core::PCWSTR = windows_core::w!("DatabasePath");
pub const CLUSREG_NAME_DRAIN_ON_SHUTDOWN: windows_core::PCWSTR = windows_core::w!("DrainOnShutdown");
pub const CLUSREG_NAME_ENABLED_EVENT_LOGS: windows_core::PCWSTR = windows_core::w!("EnabledEventLogs");
pub const CLUSREG_NAME_FAILOVER_MOVE_MIGRATION_TYPE: windows_core::PCWSTR = windows_core::w!("FailoverMoveMigrationType");
pub const CLUSREG_NAME_FILESHR_CA_TIMEOUT: windows_core::PCWSTR = windows_core::w!("CATimeout");
pub const CLUSREG_NAME_FILESHR_HIDE_SUBDIR_SHARES: windows_core::PCWSTR = windows_core::w!("HideSubDirShares");
pub const CLUSREG_NAME_FILESHR_IS_DFS_ROOT: windows_core::PCWSTR = windows_core::w!("IsDfsRoot");
pub const CLUSREG_NAME_FILESHR_MAX_USERS: windows_core::PCWSTR = windows_core::w!("MaxUsers");
pub const CLUSREG_NAME_FILESHR_PATH: windows_core::PCWSTR = windows_core::w!("Path");
pub const CLUSREG_NAME_FILESHR_QOS_FLOWSCOPE: windows_core::PCWSTR = windows_core::w!("QosFlowScope");
pub const CLUSREG_NAME_FILESHR_QOS_POLICYID: windows_core::PCWSTR = windows_core::w!("QosPolicyId");
pub const CLUSREG_NAME_FILESHR_REMARK: windows_core::PCWSTR = windows_core::w!("Remark");
pub const CLUSREG_NAME_FILESHR_SD: windows_core::PCWSTR = windows_core::w!("Security Descriptor");
pub const CLUSREG_NAME_FILESHR_SERVER_NAME: windows_core::PCWSTR = windows_core::w!("ServerName");
pub const CLUSREG_NAME_FILESHR_SHARE_FLAGS: windows_core::PCWSTR = windows_core::w!("ShareFlags");
pub const CLUSREG_NAME_FILESHR_SHARE_NAME: windows_core::PCWSTR = windows_core::w!("ShareName");
pub const CLUSREG_NAME_FILESHR_SHARE_SUBDIRS: windows_core::PCWSTR = windows_core::w!("ShareSubDirs");
pub const CLUSREG_NAME_FIXQUORUM: windows_core::PCWSTR = windows_core::w!("FixQuorum");
pub const CLUSREG_NAME_FSWITNESS_ARB_DELAY: windows_core::PCWSTR = windows_core::w!("ArbitrationDelay");
pub const CLUSREG_NAME_FSWITNESS_IMPERSONATE_CNO: windows_core::PCWSTR = windows_core::w!("ImpersonateCNO");
pub const CLUSREG_NAME_FSWITNESS_SHARE_PATH: windows_core::PCWSTR = windows_core::w!("SharePath");
pub const CLUSREG_NAME_FUNCTIONAL_LEVEL: windows_core::PCWSTR = windows_core::w!("ClusterFunctionalLevel");
pub const CLUSREG_NAME_GENAPP_COMMAND_LINE: windows_core::PCWSTR = windows_core::w!("CommandLine");
pub const CLUSREG_NAME_GENAPP_CURRENT_DIRECTORY: windows_core::PCWSTR = windows_core::w!("CurrentDirectory");
pub const CLUSREG_NAME_GENAPP_USE_NETWORK_NAME: windows_core::PCWSTR = windows_core::w!("UseNetworkName");
pub const CLUSREG_NAME_GENSCRIPT_SCRIPT_FILEPATH: windows_core::PCWSTR = windows_core::w!("ScriptFilepath");
pub const CLUSREG_NAME_GENSVC_SERVICE_NAME: windows_core::PCWSTR = windows_core::w!("ServiceName");
pub const CLUSREG_NAME_GENSVC_STARTUP_PARAMS: windows_core::PCWSTR = windows_core::w!("StartupParameters");
pub const CLUSREG_NAME_GENSVC_USE_NETWORK_NAME: windows_core::PCWSTR = windows_core::w!("UseNetworkName");
pub const CLUSREG_NAME_GPUP_DEVICE_ALLOCATIONS: windows_core::PCWSTR = windows_core::w!("GpupDeviceAllocations");
pub const CLUSREG_NAME_GROUPSET_AVAILABILITY_SET_INDEX_TO_NODE_MAPPING: windows_core::PCWSTR = windows_core::w!("NodeDomainInfo");
pub const CLUSREG_NAME_GROUPSET_FAULT_DOMAINS: windows_core::PCWSTR = windows_core::w!("FaultDomains");
pub const CLUSREG_NAME_GROUPSET_IS_AVAILABILITY_SET: windows_core::PCWSTR = windows_core::w!("IsAvailabilitySet");
pub const CLUSREG_NAME_GROUPSET_IS_GLOBAL: windows_core::PCWSTR = windows_core::w!("IsGlobal");
pub const CLUSREG_NAME_GROUPSET_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_GROUPSET_RESERVE_NODE: windows_core::PCWSTR = windows_core::w!("ReserveSpareNode");
pub const CLUSREG_NAME_GROUPSET_STARTUP_COUNT: windows_core::PCWSTR = windows_core::w!("StartupCount");
pub const CLUSREG_NAME_GROUPSET_STARTUP_DELAY: windows_core::PCWSTR = windows_core::w!("StartupDelay");
pub const CLUSREG_NAME_GROUPSET_STARTUP_SETTING: windows_core::PCWSTR = windows_core::w!("StartupSetting");
pub const CLUSREG_NAME_GROUPSET_STATUS_INFORMATION: windows_core::PCWSTR = windows_core::w!("StatusInformation");
pub const CLUSREG_NAME_GROUPSET_UPDATE_DOMAINS: windows_core::PCWSTR = windows_core::w!("UpdateDomains");
pub const CLUSREG_NAME_GROUP_DEPENDENCY_TIMEOUT: windows_core::PCWSTR = windows_core::w!("GroupDependencyTimeout");
pub const CLUSREG_NAME_GRP_ANTI_AFFINITY_CLASS_NAME: windows_core::PCWSTR = windows_core::w!("AntiAffinityClassNames");
pub const CLUSREG_NAME_GRP_CCF_EPOCH: windows_core::PCWSTR = windows_core::w!("CCFEpoch");
pub const CLUSREG_NAME_GRP_CCF_EPOCH_HIGH: windows_core::PCWSTR = windows_core::w!("CCFEpochHigh");
pub const CLUSREG_NAME_GRP_COLD_START_SETTING: windows_core::PCWSTR = windows_core::w!("ColdStartSetting");
pub const CLUSREG_NAME_GRP_DEFAULT_OWNER: windows_core::PCWSTR = windows_core::w!("DefaultOwner");
pub const CLUSREG_NAME_GRP_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_GRP_FAILBACK_TYPE: windows_core::PCWSTR = windows_core::w!("AutoFailbackType");
pub const CLUSREG_NAME_GRP_FAILBACK_WIN_END: windows_core::PCWSTR = windows_core::w!("FailbackWindowEnd");
pub const CLUSREG_NAME_GRP_FAILBACK_WIN_START: windows_core::PCWSTR = windows_core::w!("FailbackWindowStart");
pub const CLUSREG_NAME_GRP_FAILOVER_PERIOD: windows_core::PCWSTR = windows_core::w!("FailoverPeriod");
pub const CLUSREG_NAME_GRP_FAILOVER_THRESHOLD: windows_core::PCWSTR = windows_core::w!("FailoverThreshold");
pub const CLUSREG_NAME_GRP_FAULT_DOMAIN: windows_core::PCWSTR = windows_core::w!("FaultDomain");
pub const CLUSREG_NAME_GRP_LOCK_MOVE: windows_core::PCWSTR = windows_core::w!("LockedFromMoving");
pub const CLUSREG_NAME_GRP_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_GRP_PERSISTENT_STATE: windows_core::PCWSTR = windows_core::w!("PersistentState");
pub const CLUSREG_NAME_GRP_PLACEMENT_OPTIONS: windows_core::PCWSTR = windows_core::w!("PlacementOptions");
pub const CLUSREG_NAME_GRP_PREFERRED_SITE: windows_core::PCWSTR = windows_core::w!("PreferredSite");
pub const CLUSREG_NAME_GRP_PRIORITY: windows_core::PCWSTR = windows_core::w!("Priority");
pub const CLUSREG_NAME_GRP_RESILIENCY_PERIOD: windows_core::PCWSTR = windows_core::w!("ResiliencyPeriod");
pub const CLUSREG_NAME_GRP_START_DELAY: windows_core::PCWSTR = windows_core::w!("GroupStartDelay");
pub const CLUSREG_NAME_GRP_STATUS_INFORMATION: windows_core::PCWSTR = windows_core::w!("StatusInformation");
pub const CLUSREG_NAME_GRP_TYPE: windows_core::PCWSTR = windows_core::w!("GroupType");
pub const CLUSREG_NAME_GRP_UPDATE_DOMAIN: windows_core::PCWSTR = windows_core::w!("UpdateDomain");
pub const CLUSREG_NAME_IGNORE_PERSISTENT_STATE: windows_core::PCWSTR = windows_core::w!("IgnorePersistentStateOnStartup");
pub const CLUSREG_NAME_IPADDR_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_IPADDR_DHCP_ADDRESS: windows_core::PCWSTR = windows_core::w!("DhcpAddress");
pub const CLUSREG_NAME_IPADDR_DHCP_SERVER: windows_core::PCWSTR = windows_core::w!("DhcpServer");
pub const CLUSREG_NAME_IPADDR_DHCP_SUBNET_MASK: windows_core::PCWSTR = windows_core::w!("DhcpSubnetMask");
pub const CLUSREG_NAME_IPADDR_ENABLE_DHCP: windows_core::PCWSTR = windows_core::w!("EnableDhcp");
pub const CLUSREG_NAME_IPADDR_ENABLE_NETBIOS: windows_core::PCWSTR = windows_core::w!("EnableNetBIOS");
pub const CLUSREG_NAME_IPADDR_LEASE_OBTAINED_TIME: windows_core::PCWSTR = windows_core::w!("LeaseObtainedTime");
pub const CLUSREG_NAME_IPADDR_LEASE_TERMINATES_TIME: windows_core::PCWSTR = windows_core::w!("LeaseExpiresTime");
pub const CLUSREG_NAME_IPADDR_NETWORK: windows_core::PCWSTR = windows_core::w!("Network");
pub const CLUSREG_NAME_IPADDR_OVERRIDE_ADDRMATCH: windows_core::PCWSTR = windows_core::w!("OverrideAddressMatch");
pub const CLUSREG_NAME_IPADDR_PROBE_FAILURE_THRESHOLD: windows_core::PCWSTR = windows_core::w!("ProbeFailureThreshold");
pub const CLUSREG_NAME_IPADDR_PROBE_PORT: windows_core::PCWSTR = windows_core::w!("ProbePort");
pub const CLUSREG_NAME_IPADDR_SHARED_NETNAME: windows_core::PCWSTR = windows_core::w!("SharedNetname");
pub const CLUSREG_NAME_IPADDR_SUBNET_MASK: windows_core::PCWSTR = windows_core::w!("SubnetMask");
pub const CLUSREG_NAME_IPADDR_T1: windows_core::PCWSTR = windows_core::w!("T1");
pub const CLUSREG_NAME_IPADDR_T2: windows_core::PCWSTR = windows_core::w!("T2");
pub const CLUSREG_NAME_IPV6_NATIVE_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_IPV6_NATIVE_NETWORK: windows_core::PCWSTR = windows_core::w!("Network");
pub const CLUSREG_NAME_IPV6_NATIVE_PREFIX_LENGTH: windows_core::PCWSTR = windows_core::w!("PrefixLength");
pub const CLUSREG_NAME_IPV6_TUNNEL_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_IPV6_TUNNEL_TUNNELTYPE: windows_core::PCWSTR = windows_core::w!("TunnelType");
pub const CLUSREG_NAME_LAST_RECENT_EVENTS_RESET_TIME: windows_core::PCWSTR = windows_core::w!("RecentEventsResetTime");
pub const CLUSREG_NAME_LOG_FILE_PATH: windows_core::PCWSTR = windows_core::w!("LogFilePath");
pub const CLUSREG_NAME_MESSAGE_BUFFER_LENGTH: windows_core::PCWSTR = windows_core::w!("MessageBufferLength");
pub const CLUSREG_NAME_MIXED_MODE: windows_core::PCWSTR = windows_core::w!("MixedMode");
pub const CLUSREG_NAME_NETFT_IPSEC_ENABLED: windows_core::PCWSTR = windows_core::w!("NetftIPSecEnabled");
pub const CLUSREG_NAME_NETIFACE_ADAPTER_ID: windows_core::PCWSTR = windows_core::w!("AdapterId");
pub const CLUSREG_NAME_NETIFACE_ADAPTER_NAME: windows_core::PCWSTR = windows_core::w!("Adapter");
pub const CLUSREG_NAME_NETIFACE_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_NETIFACE_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_NETIFACE_DHCP_ENABLED: windows_core::PCWSTR = windows_core::w!("DhcpEnabled");
pub const CLUSREG_NAME_NETIFACE_IPV4_ADDRESSES: windows_core::PCWSTR = windows_core::w!("IPv4Addresses");
pub const CLUSREG_NAME_NETIFACE_IPV6_ADDRESSES: windows_core::PCWSTR = windows_core::w!("IPv6Addresses");
pub const CLUSREG_NAME_NETIFACE_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_NETIFACE_NETWORK: windows_core::PCWSTR = windows_core::w!("Network");
pub const CLUSREG_NAME_NETIFACE_NODE: windows_core::PCWSTR = windows_core::w!("Node");
pub const CLUSREG_NAME_NETNAME_AD_AWARE: windows_core::PCWSTR = windows_core::w!("ADAware");
pub const CLUSREG_NAME_NETNAME_ALIASES: windows_core::PCWSTR = windows_core::w!("Aliases");
pub const CLUSREG_NAME_NETNAME_CONTAINERGUID: windows_core::PCWSTR = windows_core::w!("CryptoContainerGUID");
pub const CLUSREG_NAME_NETNAME_CREATING_DC: windows_core::PCWSTR = windows_core::w!("CreatingDC");
pub const CLUSREG_NAME_NETNAME_DNN_DISABLE_CLONES: windows_core::PCWSTR = windows_core::w!("DisableClones");
pub const CLUSREG_NAME_NETNAME_DNS_NAME: windows_core::PCWSTR = windows_core::w!("DnsName");
pub const CLUSREG_NAME_NETNAME_DNS_SUFFIX: windows_core::PCWSTR = windows_core::w!("DnsSuffix");
pub const CLUSREG_NAME_NETNAME_EXCLUDE_NETWORKS: windows_core::PCWSTR = windows_core::w!("ExcludeNetworks");
pub const CLUSREG_NAME_NETNAME_HOST_TTL: windows_core::PCWSTR = windows_core::w!("HostRecordTTL");
pub const CLUSREG_NAME_NETNAME_IN_USE_NETWORKS: windows_core::PCWSTR = windows_core::w!("InUseNetworks");
pub const CLUSREG_NAME_NETNAME_LAST_DNS_UPDATE: windows_core::PCWSTR = windows_core::w!("LastDNSUpdateTime");
pub const CLUSREG_NAME_NETNAME_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_NETNAME_OBJECT_ID: windows_core::PCWSTR = windows_core::w!("ObjectGUID");
pub const CLUSREG_NAME_NETNAME_PUBLISH_PTR: windows_core::PCWSTR = windows_core::w!("PublishPTRRecords");
pub const CLUSREG_NAME_NETNAME_REGISTER_ALL_IP: windows_core::PCWSTR = windows_core::w!("RegisterAllProvidersIP");
pub const CLUSREG_NAME_NETNAME_REMAP_PIPE_NAMES: windows_core::PCWSTR = windows_core::w!("RemapPipeNames");
pub const CLUSREG_NAME_NETNAME_REMOVEVCO_ONDELETE: windows_core::PCWSTR = windows_core::w!("DeleteVcoOnResCleanup");
pub const CLUSREG_NAME_NETNAME_RESOURCE_DATA: windows_core::PCWSTR = windows_core::w!("ResourceData");
pub const CLUSREG_NAME_NETNAME_STATUS_DNS: windows_core::PCWSTR = windows_core::w!("StatusDNS");
pub const CLUSREG_NAME_NETNAME_STATUS_KERBEROS: windows_core::PCWSTR = windows_core::w!("StatusKerberos");
pub const CLUSREG_NAME_NETNAME_STATUS_NETBIOS: windows_core::PCWSTR = windows_core::w!("StatusNetBIOS");
pub const CLUSREG_NAME_NETNAME_VCO_CONTAINER: windows_core::PCWSTR = windows_core::w!("VcoContainer");
pub const CLUSREG_NAME_NET_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_NET_ADDRESS_MASK: windows_core::PCWSTR = windows_core::w!("AddressMask");
pub const CLUSREG_NAME_NET_AUTOMETRIC: windows_core::PCWSTR = windows_core::w!("AutoMetric");
pub const CLUSREG_NAME_NET_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_NET_IPV4_ADDRESSES: windows_core::PCWSTR = windows_core::w!("IPv4Addresses");
pub const CLUSREG_NAME_NET_IPV4_PREFIXLENGTHS: windows_core::PCWSTR = windows_core::w!("IPv4PrefixLengths");
pub const CLUSREG_NAME_NET_IPV6_ADDRESSES: windows_core::PCWSTR = windows_core::w!("IPv6Addresses");
pub const CLUSREG_NAME_NET_IPV6_PREFIXLENGTHS: windows_core::PCWSTR = windows_core::w!("IPv6PrefixLengths");
pub const CLUSREG_NAME_NET_METRIC: windows_core::PCWSTR = windows_core::w!("Metric");
pub const CLUSREG_NAME_NET_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_NET_RDMA_CAPABLE: windows_core::PCWSTR = windows_core::w!("RdmaCapable");
pub const CLUSREG_NAME_NET_ROLE: windows_core::PCWSTR = windows_core::w!("Role");
pub const CLUSREG_NAME_NET_RSS_CAPABLE: windows_core::PCWSTR = windows_core::w!("RssCapable");
pub const CLUSREG_NAME_NET_SPEED: windows_core::PCWSTR = windows_core::w!("LinkSpeed");
pub const CLUSREG_NAME_NODE_BUILD_NUMBER: windows_core::PCWSTR = windows_core::w!("BuildNumber");
pub const CLUSREG_NAME_NODE_CSDVERSION: windows_core::PCWSTR = windows_core::w!("CSDVersion");
pub const CLUSREG_NAME_NODE_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_NODE_DRAIN_STATUS: windows_core::PCWSTR = windows_core::w!("NodeDrainStatus");
pub const CLUSREG_NAME_NODE_DRAIN_TARGET: windows_core::PCWSTR = windows_core::w!("NodeDrainTarget");
pub const CLUSREG_NAME_NODE_DYNAMIC_WEIGHT: windows_core::PCWSTR = windows_core::w!("DynamicWeight");
pub const CLUSREG_NAME_NODE_FAULT_DOMAIN: windows_core::PCWSTR = windows_core::w!("FaultDomain");
pub const CLUSREG_NAME_NODE_FDID: windows_core::PCWSTR = windows_core::w!("FaultDomainId");
pub const CLUSREG_NAME_NODE_HIGHEST_VERSION: windows_core::PCWSTR = windows_core::w!("NodeHighestVersion");
pub const CLUSREG_NAME_NODE_IS_PRIMARY: windows_core::PCWSTR = windows_core::w!("IsPrimary");
pub const CLUSREG_NAME_NODE_LOWEST_VERSION: windows_core::PCWSTR = windows_core::w!("NodeLowestVersion");
pub const CLUSREG_NAME_NODE_MAJOR_VERSION: windows_core::PCWSTR = windows_core::w!("MajorVersion");
pub const CLUSREG_NAME_NODE_MANUFACTURER: windows_core::PCWSTR = windows_core::w!("Manufacturer");
pub const CLUSREG_NAME_NODE_MINOR_VERSION: windows_core::PCWSTR = windows_core::w!("MinorVersion");
pub const CLUSREG_NAME_NODE_MODEL: windows_core::PCWSTR = windows_core::w!("Model");
pub const CLUSREG_NAME_NODE_NAME: windows_core::PCWSTR = windows_core::w!("NodeName");
pub const CLUSREG_NAME_NODE_NEEDS_PQ: windows_core::PCWSTR = windows_core::w!("NeedsPreventQuorum");
pub const CLUSREG_NAME_NODE_SERIALNUMBER: windows_core::PCWSTR = windows_core::w!("SerialNumber");
pub const CLUSREG_NAME_NODE_STATUS_INFO: windows_core::PCWSTR = windows_core::w!("StatusInformation");
pub const CLUSREG_NAME_NODE_UNIQUEID: windows_core::PCWSTR = windows_core::w!("UniqueID");
pub const CLUSREG_NAME_NODE_WEIGHT: windows_core::PCWSTR = windows_core::w!("NodeWeight");
pub const CLUSREG_NAME_PHYSDISK_CSVBLOCKCACHE: windows_core::PCWSTR = windows_core::w!("EnableBlockCache");
pub const CLUSREG_NAME_PHYSDISK_CSVSNAPSHOTAGELIMIT: windows_core::PCWSTR = windows_core::w!("SnapshotAgeLimit");
pub const CLUSREG_NAME_PHYSDISK_CSVSNAPSHOTDIFFAREASIZE: windows_core::PCWSTR = windows_core::w!("SnapshotDiffSize");
pub const CLUSREG_NAME_PHYSDISK_CSVWRITETHROUGH: windows_core::PCWSTR = windows_core::w!("CsvEnforceWriteThrough");
pub const CLUSREG_NAME_PHYSDISK_DISKARBINTERVAL: windows_core::PCWSTR = windows_core::w!("DiskArbInterval");
pub const CLUSREG_NAME_PHYSDISK_DISKARBTYPE: windows_core::PCWSTR = windows_core::w!("DiskArbType");
pub const CLUSREG_NAME_PHYSDISK_DISKGUID: windows_core::PCWSTR = windows_core::w!("DiskGuid");
pub const CLUSREG_NAME_PHYSDISK_DISKIDGUID: windows_core::PCWSTR = windows_core::w!("DiskIdGuid");
pub const CLUSREG_NAME_PHYSDISK_DISKIDTYPE: windows_core::PCWSTR = windows_core::w!("DiskIdType");
pub const CLUSREG_NAME_PHYSDISK_DISKIODELAY: windows_core::PCWSTR = windows_core::w!("MaxIoLatency");
pub const CLUSREG_NAME_PHYSDISK_DISKPATH: windows_core::PCWSTR = windows_core::w!("DiskPath");
pub const CLUSREG_NAME_PHYSDISK_DISKRECOVERYACTION: windows_core::PCWSTR = windows_core::w!("DiskRecoveryAction");
pub const CLUSREG_NAME_PHYSDISK_DISKRELOAD: windows_core::PCWSTR = windows_core::w!("DiskReload");
pub const CLUSREG_NAME_PHYSDISK_DISKRUNCHKDSK: windows_core::PCWSTR = windows_core::w!("DiskRunChkDsk");
pub const CLUSREG_NAME_PHYSDISK_DISKSIGNATURE: windows_core::PCWSTR = windows_core::w!("DiskSignature");
pub const CLUSREG_NAME_PHYSDISK_DISKUNIQUEIDS: windows_core::PCWSTR = windows_core::w!("DiskUniqueIds");
pub const CLUSREG_NAME_PHYSDISK_DISKVOLUMEINFO: windows_core::PCWSTR = windows_core::w!("DiskVolumeInfo");
pub const CLUSREG_NAME_PHYSDISK_FASTONLINEARBITRATE: windows_core::PCWSTR = windows_core::w!("FastOnlineArbitrate");
pub const CLUSREG_NAME_PHYSDISK_MAINTMODE: windows_core::PCWSTR = windows_core::w!("MaintenanceMode");
pub const CLUSREG_NAME_PHYSDISK_MIGRATEFIXUP: windows_core::PCWSTR = windows_core::w!("MigrateDriveLetters");
pub const CLUSREG_NAME_PHYSDISK_SPACEIDGUID: windows_core::PCWSTR = windows_core::w!("VirtualDiskId");
pub const CLUSREG_NAME_PHYSDISK_VOLSNAPACTIVATETIMEOUT: windows_core::PCWSTR = windows_core::w!("VolsnapActivateTimeout");
pub const CLUSREG_NAME_PLACEMENT_OPTIONS: windows_core::PCWSTR = windows_core::w!("PlacementOptions");
pub const CLUSREG_NAME_PLUMB_ALL_CROSS_SUBNET_ROUTES: windows_core::PCWSTR = windows_core::w!("PlumbAllCrossSubnetRoutes");
pub const CLUSREG_NAME_PREVENTQUORUM: windows_core::PCWSTR = windows_core::w!("PreventQuorum");
pub const CLUSREG_NAME_PRTSPOOL_DEFAULT_SPOOL_DIR: windows_core::PCWSTR = windows_core::w!("DefaultSpoolDirectory");
pub const CLUSREG_NAME_PRTSPOOL_TIMEOUT: windows_core::PCWSTR = windows_core::w!("JobCompletionTimeout");
pub const CLUSREG_NAME_QUARANTINE_DURATION: windows_core::PCWSTR = windows_core::w!("QuarantineDuration");
pub const CLUSREG_NAME_QUARANTINE_THRESHOLD: windows_core::PCWSTR = windows_core::w!("QuarantineThreshold");
pub const CLUSREG_NAME_QUORUM_ARBITRATION_TIMEOUT: windows_core::PCWSTR = windows_core::w!("QuorumArbitrationTimeMax");
pub const CLUSREG_NAME_RESILIENCY_DEFAULT_SECONDS: windows_core::PCWSTR = windows_core::w!("ResiliencyDefaultPeriod");
pub const CLUSREG_NAME_RESILIENCY_LEVEL: windows_core::PCWSTR = windows_core::w!("ResiliencyLevel");
pub const CLUSREG_NAME_RESTYPE_ADMIN_EXTENSIONS: windows_core::PCWSTR = windows_core::w!("AdminExtensions");
pub const CLUSREG_NAME_RESTYPE_DEADLOCK_TIMEOUT: windows_core::PCWSTR = windows_core::w!("DeadlockTimeout");
pub const CLUSREG_NAME_RESTYPE_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_RESTYPE_DLL_NAME: windows_core::PCWSTR = windows_core::w!("DllName");
pub const CLUSREG_NAME_RESTYPE_DUMP_LOG_QUERY: windows_core::PCWSTR = windows_core::w!("DumpLogQuery");
pub const CLUSREG_NAME_RESTYPE_DUMP_POLICY: windows_core::PCWSTR = windows_core::w!("DumpPolicy");
pub const CLUSREG_NAME_RESTYPE_DUMP_SERVICES: windows_core::PCWSTR = windows_core::w!("DumpServices");
pub const CLUSREG_NAME_RESTYPE_ENABLED_EVENT_LOGS: windows_core::PCWSTR = windows_core::w!("EnabledEventLogs");
pub const CLUSREG_NAME_RESTYPE_IS_ALIVE: windows_core::PCWSTR = windows_core::w!("IsAlivePollInterval");
pub const CLUSREG_NAME_RESTYPE_LOOKS_ALIVE: windows_core::PCWSTR = windows_core::w!("LooksAlivePollInterval");
pub const CLUSREG_NAME_RESTYPE_MAX_MONITORS: windows_core::PCWSTR = windows_core::w!("MaximumMonitors");
pub const CLUSREG_NAME_RESTYPE_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_RESTYPE_PENDING_TIMEOUT: windows_core::PCWSTR = windows_core::w!("PendingTimeout");
pub const CLUSREG_NAME_RESTYPE_WPR_PROFILES: windows_core::PCWSTR = windows_core::w!("WprProfiles");
pub const CLUSREG_NAME_RESTYPE_WPR_START_AFTER: windows_core::PCWSTR = windows_core::w!("WprStartAfter");
pub const CLUSREG_NAME_RES_DATA1: windows_core::PCWSTR = windows_core::w!("ResourceSpecificData1");
pub const CLUSREG_NAME_RES_DATA2: windows_core::PCWSTR = windows_core::w!("ResourceSpecificData2");
pub const CLUSREG_NAME_RES_DEADLOCK_TIMEOUT: windows_core::PCWSTR = windows_core::w!("DeadlockTimeout");
pub const CLUSREG_NAME_RES_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_RES_EMBEDDED_FAILURE_ACTION: windows_core::PCWSTR = windows_core::w!("EmbeddedFailureAction");
pub const CLUSREG_NAME_RES_IS_ALIVE: windows_core::PCWSTR = windows_core::w!("IsAlivePollInterval");
pub const CLUSREG_NAME_RES_LAST_OPERATION_STATUS_CODE: windows_core::PCWSTR = windows_core::w!("LastOperationStatusCode");
pub const CLUSREG_NAME_RES_LOOKS_ALIVE: windows_core::PCWSTR = windows_core::w!("LooksAlivePollInterval");
pub const CLUSREG_NAME_RES_MONITOR_PID: windows_core::PCWSTR = windows_core::w!("MonitorProcessId");
pub const CLUSREG_NAME_RES_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_RES_PENDING_TIMEOUT: windows_core::PCWSTR = windows_core::w!("PendingTimeout");
pub const CLUSREG_NAME_RES_PERSISTENT_STATE: windows_core::PCWSTR = windows_core::w!("PersistentState");
pub const CLUSREG_NAME_RES_RESTART_ACTION: windows_core::PCWSTR = windows_core::w!("RestartAction");
pub const CLUSREG_NAME_RES_RESTART_DELAY: windows_core::PCWSTR = windows_core::w!("RestartDelay");
pub const CLUSREG_NAME_RES_RESTART_PERIOD: windows_core::PCWSTR = windows_core::w!("RestartPeriod");
pub const CLUSREG_NAME_RES_RESTART_THRESHOLD: windows_core::PCWSTR = windows_core::w!("RestartThreshold");
pub const CLUSREG_NAME_RES_RETRY_PERIOD_ON_FAILURE: windows_core::PCWSTR = windows_core::w!("RetryPeriodOnFailure");
pub const CLUSREG_NAME_RES_SEPARATE_MONITOR: windows_core::PCWSTR = windows_core::w!("SeparateMonitor");
pub const CLUSREG_NAME_RES_STATUS: windows_core::PCWSTR = windows_core::w!("ResourceSpecificStatus");
pub const CLUSREG_NAME_RES_STATUS_INFORMATION: windows_core::PCWSTR = windows_core::w!("StatusInformation");
pub const CLUSREG_NAME_RES_TYPE: windows_core::PCWSTR = windows_core::w!("Type");
pub const CLUSREG_NAME_ROUTE_HISTORY_LENGTH: windows_core::PCWSTR = windows_core::w!("RouteHistoryLength");
pub const CLUSREG_NAME_SAME_SUBNET_DELAY: windows_core::PCWSTR = windows_core::w!("SameSubnetDelay");
pub const CLUSREG_NAME_SAME_SUBNET_THRESHOLD: windows_core::PCWSTR = windows_core::w!("SameSubnetThreshold");
pub const CLUSREG_NAME_SHUTDOWN_TIMEOUT_MINUTES: windows_core::PCWSTR = windows_core::w!("ShutdownTimeoutInMinutes");
pub const CLUSREG_NAME_SOFS_SMBASYMMETRYMODE: windows_core::PCWSTR = windows_core::w!("SmbAsymmetryMode");
pub const CLUSREG_NAME_START_MEMORY: windows_core::PCWSTR = windows_core::w!("StartMemory");
pub const CLUSREG_NAME_STORAGESPACE_DESCRIPTION: windows_core::PCWSTR = windows_core::w!("VirtualDiskDescription");
pub const CLUSREG_NAME_STORAGESPACE_HEALTH: windows_core::PCWSTR = windows_core::w!("VirtualDiskHealth");
pub const CLUSREG_NAME_STORAGESPACE_NAME: windows_core::PCWSTR = windows_core::w!("VirtualDiskName");
pub const CLUSREG_NAME_STORAGESPACE_POOLARBITRATE: windows_core::PCWSTR = windows_core::w!("Arbitrate");
pub const CLUSREG_NAME_STORAGESPACE_POOLCONSUMEDCAPACITY: windows_core::PCWSTR = windows_core::w!("ConsumedCapacity");
pub const CLUSREG_NAME_STORAGESPACE_POOLDESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_STORAGESPACE_POOLDRIVEIDS: windows_core::PCWSTR = windows_core::w!("DriveIds");
pub const CLUSREG_NAME_STORAGESPACE_POOLHEALTH: windows_core::PCWSTR = windows_core::w!("Health");
pub const CLUSREG_NAME_STORAGESPACE_POOLIDGUID: windows_core::PCWSTR = windows_core::w!("PoolId");
pub const CLUSREG_NAME_STORAGESPACE_POOLNAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_STORAGESPACE_POOLQUORUMSHARE: windows_core::PCWSTR = windows_core::w!("PoolQuorumShare");
pub const CLUSREG_NAME_STORAGESPACE_POOLQUORUMUSERACCOUNT: windows_core::PCWSTR = windows_core::w!("PoolQuorumUserAccount");
pub const CLUSREG_NAME_STORAGESPACE_POOLREEVALTIMEOUT: windows_core::PCWSTR = windows_core::w!("ReEvaluatePlacementTimeout");
pub const CLUSREG_NAME_STORAGESPACE_POOLSTATE: windows_core::PCWSTR = windows_core::w!("State");
pub const CLUSREG_NAME_STORAGESPACE_POOLTOTALCAPACITY: windows_core::PCWSTR = windows_core::w!("TotalCapacity");
pub const CLUSREG_NAME_STORAGESPACE_PROVISIONING: windows_core::PCWSTR = windows_core::w!("VirtualDiskProvisioning");
pub const CLUSREG_NAME_STORAGESPACE_RESILIENCYCOLUMNS: windows_core::PCWSTR = windows_core::w!("VirtualDiskResiliencyColumns");
pub const CLUSREG_NAME_STORAGESPACE_RESILIENCYINTERLEAVE: windows_core::PCWSTR = windows_core::w!("VirtualDiskResiliencyInterleave");
pub const CLUSREG_NAME_STORAGESPACE_RESILIENCYTYPE: windows_core::PCWSTR = windows_core::w!("VirtualDiskResiliencyType");
pub const CLUSREG_NAME_STORAGESPACE_STATE: windows_core::PCWSTR = windows_core::w!("VirtualDiskState");
pub const CLUSREG_NAME_UPGRADE_VERSION: windows_core::PCWSTR = windows_core::w!("ClusterUpgradeVersion");
pub const CLUSREG_NAME_VIP_ADAPTER_NAME: windows_core::PCWSTR = windows_core::w!("AdapterName");
pub const CLUSREG_NAME_VIP_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_VIP_PREFIX_LENGTH: windows_core::PCWSTR = windows_core::w!("PrefixLength");
pub const CLUSREG_NAME_VIP_RDID: windows_core::PCWSTR = windows_core::w!("RDID");
pub const CLUSREG_NAME_VIP_VSID: windows_core::PCWSTR = windows_core::w!("VSID");
pub const CLUSREG_NAME_VIRTUAL_NUMA_COUNT: windows_core::PCWSTR = windows_core::w!("VirtualNumaCount");
pub const CLUSREG_NAME_VSSTASK_APPNAME: windows_core::PCWSTR = windows_core::w!("ApplicationName");
pub const CLUSREG_NAME_VSSTASK_APPPARAMS: windows_core::PCWSTR = windows_core::w!("ApplicationParams");
pub const CLUSREG_NAME_VSSTASK_CURRENTDIRECTORY: windows_core::PCWSTR = windows_core::w!("CurrentDirectory");
pub const CLUSREG_NAME_VSSTASK_TRIGGERARRAY: windows_core::PCWSTR = windows_core::w!("TriggerArray");
pub const CLUSREG_NAME_WINS_BACKUP_PATH: windows_core::PCWSTR = windows_core::w!("BackupPath");
pub const CLUSREG_NAME_WINS_DATABASE_PATH: windows_core::PCWSTR = windows_core::w!("DatabasePath");
pub const CLUSREG_NAME_WITNESS_DYNAMIC_WEIGHT: windows_core::PCWSTR = windows_core::w!("WitnessDynamicWeight");
pub const CLUSREG_READ_ERROR: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(9i32);
pub const CLUSREG_READ_KEY: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(7i32);
pub const CLUSREG_READ_VALUE: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(8i32);
pub const CLUSREG_SET_KEY_SECURITY: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(5i32);
pub const CLUSREG_SET_VALUE: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(1i32);
pub const CLUSREG_VALUE_DELETED: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(6i32);
pub const CLUSRESDLL_STATUS_DO_NOT_COLLECT_WER_REPORT: u32 = 1073741824u32;
pub const CLUSRESDLL_STATUS_DUMP_NOW: u32 = 2147483648u32;
pub const CLUSRESDLL_STATUS_INSUFFICIENT_MEMORY: u32 = 16u32;
pub const CLUSRESDLL_STATUS_INSUFFICIENT_OTHER_RESOURCES: u32 = 64u32;
pub const CLUSRESDLL_STATUS_INSUFFICIENT_PROCESSOR: u32 = 32u32;
pub const CLUSRESDLL_STATUS_INVALID_PARAMETERS: u32 = 128u32;
pub const CLUSRESDLL_STATUS_NETWORK_NOT_AVAILABLE: u32 = 256u32;
pub const CLUSRESDLL_STATUS_OFFLINE_BUSY: u32 = 1u32;
pub const CLUSRESDLL_STATUS_OFFLINE_DESTINATION_REJECTED: u32 = 8u32;
pub const CLUSRESDLL_STATUS_OFFLINE_DESTINATION_THROTTLED: u32 = 4u32;
pub const CLUSRESDLL_STATUS_OFFLINE_SOURCE_THROTTLED: u32 = 2u32;
pub const CLUSRES_DISABLE_WPR_WATCHDOG_FOR_OFFLINE_CALLS: u32 = 2u32;
pub const CLUSRES_DISABLE_WPR_WATCHDOG_FOR_ONLINE_CALLS: u32 = 1u32;
pub const CLUSRES_NAME_GET_OPERATION_CONTEXT_FLAGS: windows_core::PCWSTR = windows_core::w!("Flags");
pub const CLUSRES_STATUS_APPLICATION_READY: u64 = 256u64;
pub const CLUSRES_STATUS_EMBEDDED_FAILURE: u64 = 2u64;
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_CPU: u64 = 4u64;
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_GENERIC_RESOURCES: u64 = 16u64;
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_MEMORY: u64 = 8u64;
pub const CLUSRES_STATUS_LOCKED_MODE: u64 = 1u64;
pub const CLUSRES_STATUS_NETWORK_FAILURE: u64 = 32u64;
pub const CLUSRES_STATUS_OFFLINE_NOT_LOCAL_DISK_OWNER: u64 = 512u64;
pub const CLUSRES_STATUS_OS_HEARTBEAT: u64 = 128u64;
pub const CLUSRES_STATUS_UNMONITORED: u64 = 64u64;
pub const CLUSTERSET_OBJECT_TYPE_DATABASE: CLUSTERSET_OBJECT_TYPE = CLUSTERSET_OBJECT_TYPE(3i32);
pub const CLUSTERSET_OBJECT_TYPE_MEMBER: CLUSTERSET_OBJECT_TYPE = CLUSTERSET_OBJECT_TYPE(1i32);
pub const CLUSTERSET_OBJECT_TYPE_NONE: CLUSTERSET_OBJECT_TYPE = CLUSTERSET_OBJECT_TYPE(0i32);
pub const CLUSTERSET_OBJECT_TYPE_WORKLOAD: CLUSTERSET_OBJECT_TYPE = CLUSTERSET_OBJECT_TYPE(2i32);
pub const CLUSTER_ADD_EVICT_DELAY: windows_core::PCWSTR = windows_core::w!("AddEvictDelay");
pub const CLUSTER_AVAILABILITY_SET_CONFIG_V1: u32 = 1u32;
pub const CLUSTER_CHANGE_ALL: CLUSTER_CHANGE = CLUSTER_CHANGE(-1i32);
pub const CLUSTER_CHANGE_CLUSTER_ALL_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(8191i32);
pub const CLUSTER_CHANGE_CLUSTER_COMMON_PROPERTY_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(128i32);
pub const CLUSTER_CHANGE_CLUSTER_GROUP_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(4i32);
pub const CLUSTER_CHANGE_CLUSTER_HANDLE_CLOSE_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(8i32);
pub const CLUSTER_CHANGE_CLUSTER_LOST_NOTIFICATIONS_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(512i32);
pub const CLUSTER_CHANGE_CLUSTER_MEMBERSHIP_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(2048i32);
pub const CLUSTER_CHANGE_CLUSTER_NETWORK_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(16i32);
pub const CLUSTER_CHANGE_CLUSTER_NODE_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(32i32);
pub const CLUSTER_CHANGE_CLUSTER_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(256i32);
pub const CLUSTER_CHANGE_CLUSTER_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(1073741824i32);
pub const CLUSTER_CHANGE_CLUSTER_RECONNECT: CLUSTER_CHANGE = CLUSTER_CHANGE(524288i32);
pub const CLUSTER_CHANGE_CLUSTER_RECONNECT_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(1i32);
pub const CLUSTER_CHANGE_CLUSTER_RENAME_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(1024i32);
pub const CLUSTER_CHANGE_CLUSTER_RESOURCE_TYPE_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(64i32);
pub const CLUSTER_CHANGE_CLUSTER_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(536870912i32);
pub const CLUSTER_CHANGE_CLUSTER_STATE_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(2i32);
pub const CLUSTER_CHANGE_CLUSTER_UPGRADED_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(4096i32);
pub const CLUSTER_CHANGE_GROUPSET_ALL_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(511i32);
pub const CLUSTER_CHANGE_GROUPSET_COMMON_PROPERTY_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(2i32);
pub const CLUSTER_CHANGE_GROUPSET_DELETED_v2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(1i32);
pub const CLUSTER_CHANGE_GROUPSET_DEPENDENCIES_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(64i32);
pub const CLUSTER_CHANGE_GROUPSET_DEPENDENTS_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(128i32);
pub const CLUSTER_CHANGE_GROUPSET_GROUP_ADDED: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(16i32);
pub const CLUSTER_CHANGE_GROUPSET_GROUP_REMOVED: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(32i32);
pub const CLUSTER_CHANGE_GROUPSET_HANDLE_CLOSE_v2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(256i32);
pub const CLUSTER_CHANGE_GROUPSET_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(4i32);
pub const CLUSTER_CHANGE_GROUPSET_STATE_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(8i32);
pub const CLUSTER_CHANGE_GROUP_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(16384i32);
pub const CLUSTER_CHANGE_GROUP_ALL_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(1023i32);
pub const CLUSTER_CHANGE_GROUP_COMMON_PROPERTY_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(2i32);
pub const CLUSTER_CHANGE_GROUP_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(8192i32);
pub const CLUSTER_CHANGE_GROUP_DELETED_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(1i32);
pub const CLUSTER_CHANGE_GROUP_HANDLE_CLOSE_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(512i32);
pub const CLUSTER_CHANGE_GROUP_OWNER_NODE_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(16i32);
pub const CLUSTER_CHANGE_GROUP_PREFERRED_OWNERS_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(32i32);
pub const CLUSTER_CHANGE_GROUP_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(4i32);
pub const CLUSTER_CHANGE_GROUP_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(32768i32);
pub const CLUSTER_CHANGE_GROUP_RESOURCE_ADDED_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(64i32);
pub const CLUSTER_CHANGE_GROUP_RESOURCE_GAINED_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(128i32);
pub const CLUSTER_CHANGE_GROUP_RESOURCE_LOST_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(256i32);
pub const CLUSTER_CHANGE_GROUP_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(4096i32);
pub const CLUSTER_CHANGE_GROUP_STATE_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(8i32);
pub const CLUSTER_CHANGE_HANDLE_CLOSE: CLUSTER_CHANGE = CLUSTER_CHANGE(-2147483648i32);
pub const CLUSTER_CHANGE_NETINTERFACE_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(67108864i32);
pub const CLUSTER_CHANGE_NETINTERFACE_ALL_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(31i32);
pub const CLUSTER_CHANGE_NETINTERFACE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(2i32);
pub const CLUSTER_CHANGE_NETINTERFACE_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(33554432i32);
pub const CLUSTER_CHANGE_NETINTERFACE_DELETED_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(1i32);
pub const CLUSTER_CHANGE_NETINTERFACE_HANDLE_CLOSE_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(16i32);
pub const CLUSTER_CHANGE_NETINTERFACE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(4i32);
pub const CLUSTER_CHANGE_NETINTERFACE_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(134217728i32);
pub const CLUSTER_CHANGE_NETINTERFACE_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(16777216i32);
pub const CLUSTER_CHANGE_NETINTERFACE_STATE_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(8i32);
pub const CLUSTER_CHANGE_NETWORK_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(4194304i32);
pub const CLUSTER_CHANGE_NETWORK_ALL_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(31i32);
pub const CLUSTER_CHANGE_NETWORK_COMMON_PROPERTY_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(2i32);
pub const CLUSTER_CHANGE_NETWORK_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(2097152i32);
pub const CLUSTER_CHANGE_NETWORK_DELETED_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(1i32);
pub const CLUSTER_CHANGE_NETWORK_HANDLE_CLOSE_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(16i32);
pub const CLUSTER_CHANGE_NETWORK_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(4i32);
pub const CLUSTER_CHANGE_NETWORK_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(8388608i32);
pub const CLUSTER_CHANGE_NETWORK_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(1048576i32);
pub const CLUSTER_CHANGE_NETWORK_STATE_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(8i32);
pub const CLUSTER_CHANGE_NODE_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(4i32);
pub const CLUSTER_CHANGE_NODE_ALL_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(255i32);
pub const CLUSTER_CHANGE_NODE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(4i32);
pub const CLUSTER_CHANGE_NODE_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(2i32);
pub const CLUSTER_CHANGE_NODE_DELETED_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(2i32);
pub const CLUSTER_CHANGE_NODE_GROUP_GAINED_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(32i32);
pub const CLUSTER_CHANGE_NODE_GROUP_LOST_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(64i32);
pub const CLUSTER_CHANGE_NODE_HANDLE_CLOSE_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(128i32);
pub const CLUSTER_CHANGE_NODE_NETINTERFACE_ADDED_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(1i32);
pub const CLUSTER_CHANGE_NODE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(8i32);
pub const CLUSTER_CHANGE_NODE_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(8i32);
pub const CLUSTER_CHANGE_NODE_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(1i32);
pub const CLUSTER_CHANGE_NODE_STATE_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(16i32);
pub const CLUSTER_CHANGE_QUORUM_ALL_V2: CLUSTER_CHANGE_QUORUM_V2 = CLUSTER_CHANGE_QUORUM_V2(1i32);
pub const CLUSTER_CHANGE_QUORUM_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(268435456i32);
pub const CLUSTER_CHANGE_QUORUM_STATE_V2: CLUSTER_CHANGE_QUORUM_V2 = CLUSTER_CHANGE_QUORUM_V2(1i32);
pub const CLUSTER_CHANGE_REGISTRY_ALL_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(31i32);
pub const CLUSTER_CHANGE_REGISTRY_ATTRIBUTES: CLUSTER_CHANGE = CLUSTER_CHANGE(32i32);
pub const CLUSTER_CHANGE_REGISTRY_ATTRIBUTES_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(1i32);
pub const CLUSTER_CHANGE_REGISTRY_HANDLE_CLOSE_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(16i32);
pub const CLUSTER_CHANGE_REGISTRY_NAME: CLUSTER_CHANGE = CLUSTER_CHANGE(16i32);
pub const CLUSTER_CHANGE_REGISTRY_NAME_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(2i32);
pub const CLUSTER_CHANGE_REGISTRY_SUBTREE: CLUSTER_CHANGE = CLUSTER_CHANGE(128i32);
pub const CLUSTER_CHANGE_REGISTRY_SUBTREE_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(4i32);
pub const CLUSTER_CHANGE_REGISTRY_VALUE: CLUSTER_CHANGE = CLUSTER_CHANGE(64i32);
pub const CLUSTER_CHANGE_REGISTRY_VALUE_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(8i32);
pub const CLUSTER_CHANGE_RESOURCE_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(1024i32);
pub const CLUSTER_CHANGE_RESOURCE_ALL_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(2047i32);
pub const CLUSTER_CHANGE_RESOURCE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(1i32);
pub const CLUSTER_CHANGE_RESOURCE_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(512i32);
pub const CLUSTER_CHANGE_RESOURCE_DELETED_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(128i32);
pub const CLUSTER_CHANGE_RESOURCE_DEPENDENCIES_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(16i32);
pub const CLUSTER_CHANGE_RESOURCE_DEPENDENTS_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(32i32);
pub const CLUSTER_CHANGE_RESOURCE_DLL_UPGRADED_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(256i32);
pub const CLUSTER_CHANGE_RESOURCE_HANDLE_CLOSE_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(512i32);
pub const CLUSTER_CHANGE_RESOURCE_OWNER_GROUP_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(8i32);
pub const CLUSTER_CHANGE_RESOURCE_POSSIBLE_OWNERS_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(64i32);
pub const CLUSTER_CHANGE_RESOURCE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(2i32);
pub const CLUSTER_CHANGE_RESOURCE_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(2048i32);
pub const CLUSTER_CHANGE_RESOURCE_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(256i32);
pub const CLUSTER_CHANGE_RESOURCE_STATE_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(4i32);
pub const CLUSTER_CHANGE_RESOURCE_TERMINAL_STATE_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(1024i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(131072i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_ALL_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(63i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(2i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(65536i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DELETED_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(1i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DLL_UPGRADED_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(16i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_POSSIBLE_OWNERS_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(8i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(4i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(262144i32);
pub const CLUSTER_CHANGE_SHARED_VOLUME_ADDED_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = CLUSTER_CHANGE_SHARED_VOLUME_V2(2i32);
pub const CLUSTER_CHANGE_SHARED_VOLUME_ALL_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = CLUSTER_CHANGE_SHARED_VOLUME_V2(7i32);
pub const CLUSTER_CHANGE_SHARED_VOLUME_REMOVED_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = CLUSTER_CHANGE_SHARED_VOLUME_V2(4i32);
pub const CLUSTER_CHANGE_SHARED_VOLUME_STATE_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = CLUSTER_CHANGE_SHARED_VOLUME_V2(1i32);
pub const CLUSTER_CHANGE_SPACEPORT_CUSTOM_PNP_V2: CLUSTER_CHANGE_SPACEPORT_V2 = CLUSTER_CHANGE_SPACEPORT_V2(1i32);
pub const CLUSTER_CHANGE_UPGRADE_ALL: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2(7i32);
pub const CLUSTER_CHANGE_UPGRADE_NODE_COMMIT: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2(2i32);
pub const CLUSTER_CHANGE_UPGRADE_NODE_POSTCOMMIT: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2(4i32);
pub const CLUSTER_CHANGE_UPGRADE_NODE_PREPARE: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2(1i32);
pub const CLUSTER_CLOUD_TYPE_AZURE: CLUSTER_CLOUD_TYPE = CLUSTER_CLOUD_TYPE(1i32);
pub const CLUSTER_CLOUD_TYPE_MIXED: CLUSTER_CLOUD_TYPE = CLUSTER_CLOUD_TYPE(128i32);
pub const CLUSTER_CLOUD_TYPE_NONE: CLUSTER_CLOUD_TYPE = CLUSTER_CLOUD_TYPE(0i32);
pub const CLUSTER_CLOUD_TYPE_UNKNOWN: CLUSTER_CLOUD_TYPE = CLUSTER_CLOUD_TYPE(-1i32);
pub const CLUSTER_CONFIGURED: u32 = 2u32;
pub const CLUSTER_CREATE_GROUP_INFO_VERSION: u32 = 1u32;
pub const CLUSTER_CREATE_GROUP_INFO_VERSION_1: u32 = 1u32;
pub const CLUSTER_CSA_VSS_STATE: windows_core::PCWSTR = windows_core::w!("BackupInProgress");
pub const CLUSTER_CSV_COMPATIBLE_FILTERS: windows_core::PCWSTR = windows_core::w!("SharedVolumeCompatibleFilters");
pub const CLUSTER_CSV_INCOMPATIBLE_FILTERS: windows_core::PCWSTR = windows_core::w!("SharedVolumeIncompatibleFilters");
pub const CLUSTER_DELETE_ACCESS_CONTROL_ENTRY: u32 = 2u32;
pub const CLUSTER_ENFORCED_ANTIAFFINITY: windows_core::PCWSTR = windows_core::w!("ClusterEnforcedAntiaffinity");
pub const CLUSTER_ENUM_ALL: CLUSTER_ENUM = CLUSTER_ENUM(63i32);
pub const CLUSTER_ENUM_GROUP: CLUSTER_ENUM = CLUSTER_ENUM(8i32);
pub const CLUSTER_ENUM_INTERNAL_NETWORK: CLUSTER_ENUM = CLUSTER_ENUM(-2147483648i32);
pub const CLUSTER_ENUM_ITEM_VERSION: u32 = 1u32;
pub const CLUSTER_ENUM_ITEM_VERSION_1: u32 = 1u32;
pub const CLUSTER_ENUM_NETINTERFACE: CLUSTER_ENUM = CLUSTER_ENUM(32i32);
pub const CLUSTER_ENUM_NETWORK: CLUSTER_ENUM = CLUSTER_ENUM(16i32);
pub const CLUSTER_ENUM_NODE: CLUSTER_ENUM = CLUSTER_ENUM(1i32);
pub const CLUSTER_ENUM_RESOURCE: CLUSTER_ENUM = CLUSTER_ENUM(4i32);
pub const CLUSTER_ENUM_RESTYPE: CLUSTER_ENUM = CLUSTER_ENUM(2i32);
pub const CLUSTER_ENUM_SHARED_VOLUME_GROUP: CLUSTER_ENUM = CLUSTER_ENUM(536870912i32);
pub const CLUSTER_ENUM_SHARED_VOLUME_RESOURCE: CLUSTER_ENUM = CLUSTER_ENUM(1073741824i32);
pub const CLUSTER_GROUP_ENUM_ALL: CLUSTER_GROUP_ENUM = CLUSTER_GROUP_ENUM(3i32);
pub const CLUSTER_GROUP_ENUM_CONTAINS: CLUSTER_GROUP_ENUM = CLUSTER_GROUP_ENUM(1i32);
pub const CLUSTER_GROUP_ENUM_ITEM_VERSION: u32 = 1u32;
pub const CLUSTER_GROUP_ENUM_ITEM_VERSION_1: u32 = 1u32;
pub const CLUSTER_GROUP_ENUM_NODES: CLUSTER_GROUP_ENUM = CLUSTER_GROUP_ENUM(2i32);
pub const CLUSTER_GROUP_WAIT_DELAY: windows_core::PCWSTR = windows_core::w!("ClusterGroupWaitDelay");
pub const CLUSTER_HANG_RECOVERY_ACTION_KEYNAME: windows_core::PCWSTR = windows_core::w!("HangRecoveryAction");
pub const CLUSTER_HANG_TIMEOUT_KEYNAME: windows_core::PCWSTR = windows_core::w!("ClusSvcHangTimeout");
pub const CLUSTER_HEALTH_FAULT_ARGS: u32 = 7u32;
pub const CLUSTER_HEALTH_FAULT_DESCRIPTION: u32 = 3u32;
pub const CLUSTER_HEALTH_FAULT_DESCRIPTION_LABEL: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSTER_HEALTH_FAULT_ERRORCODE: u32 = 2u32;
pub const CLUSTER_HEALTH_FAULT_ERRORCODE_LABEL: windows_core::PCWSTR = windows_core::w!("ErrorCode");
pub const CLUSTER_HEALTH_FAULT_ERRORTYPE: u32 = 1u32;
pub const CLUSTER_HEALTH_FAULT_ERRORTYPE_LABEL: windows_core::PCWSTR = windows_core::w!("ErrorType");
pub const CLUSTER_HEALTH_FAULT_FLAGS: u32 = 5u32;
pub const CLUSTER_HEALTH_FAULT_FLAGS_LABEL: windows_core::PCWSTR = windows_core::w!("Flags");
pub const CLUSTER_HEALTH_FAULT_ID: u32 = 0u32;
pub const CLUSTER_HEALTH_FAULT_ID_LABEL: windows_core::PCWSTR = windows_core::w!("Id");
pub const CLUSTER_HEALTH_FAULT_PROPERTY_NAME: windows_core::PCWSTR = windows_core::w!("ClusterHealth");
pub const CLUSTER_HEALTH_FAULT_PROVIDER: u32 = 4u32;
pub const CLUSTER_HEALTH_FAULT_PROVIDER_LABEL: windows_core::PCWSTR = windows_core::w!("Provider");
pub const CLUSTER_HEALTH_FAULT_RESERVED: u32 = 6u32;
pub const CLUSTER_HEALTH_FAULT_RESERVED_LABEL: windows_core::PCWSTR = windows_core::w!("Reserved");
pub const CLUSTER_INSTALLED: u32 = 1u32;
pub const CLUSTER_MGMT_POINT_RESTYPE_AUTO: CLUSTER_MGMT_POINT_RESTYPE = CLUSTER_MGMT_POINT_RESTYPE(0i32);
pub const CLUSTER_MGMT_POINT_RESTYPE_DNN: CLUSTER_MGMT_POINT_RESTYPE = CLUSTER_MGMT_POINT_RESTYPE(2i32);
pub const CLUSTER_MGMT_POINT_RESTYPE_SNN: CLUSTER_MGMT_POINT_RESTYPE = CLUSTER_MGMT_POINT_RESTYPE(1i32);
pub const CLUSTER_MGMT_POINT_TYPE_CNO: CLUSTER_MGMT_POINT_TYPE = CLUSTER_MGMT_POINT_TYPE(1i32);
pub const CLUSTER_MGMT_POINT_TYPE_CNO_ONLY: CLUSTER_MGMT_POINT_TYPE = CLUSTER_MGMT_POINT_TYPE(3i32);
pub const CLUSTER_MGMT_POINT_TYPE_DNS_ONLY: CLUSTER_MGMT_POINT_TYPE = CLUSTER_MGMT_POINT_TYPE(2i32);
pub const CLUSTER_MGMT_POINT_TYPE_NONE: CLUSTER_MGMT_POINT_TYPE = CLUSTER_MGMT_POINT_TYPE(0i32);
pub const CLUSTER_NAME_AUTO_BALANCER_LEVEL: windows_core::PCWSTR = windows_core::w!("AutoBalancerLevel");
pub const CLUSTER_NAME_AUTO_BALANCER_MODE: windows_core::PCWSTR = windows_core::w!("AutoBalancerMode");
pub const CLUSTER_NAME_PREFERRED_SITE: windows_core::PCWSTR = windows_core::w!("PreferredSite");
pub const CLUSTER_NETWORK_ENUM_ALL: CLUSTER_NETWORK_ENUM = CLUSTER_NETWORK_ENUM(1i32);
pub const CLUSTER_NETWORK_ENUM_NETINTERFACES: CLUSTER_NETWORK_ENUM = CLUSTER_NETWORK_ENUM(1i32);
pub const CLUSTER_NODE_ENUM_ALL: CLUSTER_NODE_ENUM = CLUSTER_NODE_ENUM(3i32);
pub const CLUSTER_NODE_ENUM_GROUPS: CLUSTER_NODE_ENUM = CLUSTER_NODE_ENUM(2i32);
pub const CLUSTER_NODE_ENUM_NETINTERFACES: CLUSTER_NODE_ENUM = CLUSTER_NODE_ENUM(1i32);
pub const CLUSTER_NODE_ENUM_PREFERRED_GROUPS: CLUSTER_NODE_ENUM = CLUSTER_NODE_ENUM(4i32);
pub const CLUSTER_NOTIFICATIONS_V1: CLUSTER_NOTIFICATIONS_VERSION = CLUSTER_NOTIFICATIONS_VERSION(1i32);
pub const CLUSTER_NOTIFICATIONS_V2: CLUSTER_NOTIFICATIONS_VERSION = CLUSTER_NOTIFICATIONS_VERSION(2i32);
pub const CLUSTER_OBJECT_TYPE_AFFINITYRULE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(16i32);
pub const CLUSTER_OBJECT_TYPE_CLUSTER: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(1i32);
pub const CLUSTER_OBJECT_TYPE_FAULTDOMAIN: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(17i32);
pub const CLUSTER_OBJECT_TYPE_GROUP: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(2i32);
pub const CLUSTER_OBJECT_TYPE_GROUPSET: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(13i32);
pub const CLUSTER_OBJECT_TYPE_NETWORK: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(6i32);
pub const CLUSTER_OBJECT_TYPE_NETWORK_INTERFACE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(5i32);
pub const CLUSTER_OBJECT_TYPE_NODE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(7i32);
pub const CLUSTER_OBJECT_TYPE_NONE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(0i32);
pub const CLUSTER_OBJECT_TYPE_QUORUM: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(9i32);
pub const CLUSTER_OBJECT_TYPE_REGISTRY: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(8i32);
pub const CLUSTER_OBJECT_TYPE_RESOURCE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(3i32);
pub const CLUSTER_OBJECT_TYPE_RESOURCE_TYPE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(4i32);
pub const CLUSTER_OBJECT_TYPE_SHARED_VOLUME: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(10i32);
pub const CLUSTER_QUORUM_LOST: CLUSTER_QUORUM_VALUE = CLUSTER_QUORUM_VALUE(1i32);
pub const CLUSTER_QUORUM_MAINTAINED: CLUSTER_QUORUM_VALUE = CLUSTER_QUORUM_VALUE(0i32);
pub const CLUSTER_REQUEST_REPLY_TIMEOUT: windows_core::PCWSTR = windows_core::w!("RequestReplyTimeout");
pub const CLUSTER_RESOURCE_DEFAULT_MONITOR: CLUSTER_RESOURCE_CREATE_FLAGS = CLUSTER_RESOURCE_CREATE_FLAGS(0i32);
pub const CLUSTER_RESOURCE_ENUM_ALL: CLUSTER_RESOURCE_ENUM = CLUSTER_RESOURCE_ENUM(7i32);
pub const CLUSTER_RESOURCE_ENUM_DEPENDS: CLUSTER_RESOURCE_ENUM = CLUSTER_RESOURCE_ENUM(1i32);
pub const CLUSTER_RESOURCE_ENUM_ITEM_VERSION: u32 = 1u32;
pub const CLUSTER_RESOURCE_ENUM_ITEM_VERSION_1: u32 = 1u32;
pub const CLUSTER_RESOURCE_ENUM_NODES: CLUSTER_RESOURCE_ENUM = CLUSTER_RESOURCE_ENUM(4i32);
pub const CLUSTER_RESOURCE_ENUM_PROVIDES: CLUSTER_RESOURCE_ENUM = CLUSTER_RESOURCE_ENUM(2i32);
pub const CLUSTER_RESOURCE_SEPARATE_MONITOR: CLUSTER_RESOURCE_CREATE_FLAGS = CLUSTER_RESOURCE_CREATE_FLAGS(1i32);
pub const CLUSTER_RESOURCE_TYPE_ENUM_ALL: CLUSTER_RESOURCE_TYPE_ENUM = CLUSTER_RESOURCE_TYPE_ENUM(3i32);
pub const CLUSTER_RESOURCE_TYPE_ENUM_NODES: CLUSTER_RESOURCE_TYPE_ENUM = CLUSTER_RESOURCE_TYPE_ENUM(1i32);
pub const CLUSTER_RESOURCE_TYPE_ENUM_RESOURCES: CLUSTER_RESOURCE_TYPE_ENUM = CLUSTER_RESOURCE_TYPE_ENUM(2i32);
pub const CLUSTER_RESOURCE_TYPE_SPECIFIC_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(32i32);
pub const CLUSTER_RESOURCE_VALID_FLAGS: CLUSTER_RESOURCE_CREATE_FLAGS = CLUSTER_RESOURCE_CREATE_FLAGS(1i32);
pub const CLUSTER_RUNNING: u32 = 16u32;
pub const CLUSTER_S2D_BUS_TYPES: windows_core::PCWSTR = windows_core::w!("S2DBusTypes");
pub const CLUSTER_S2D_CACHE_BEHAVIOR_FLAGS: windows_core::PCWSTR = windows_core::w!("S2DCacheBehavior");
pub const CLUSTER_S2D_CACHE_DESIRED_STATE: windows_core::PCWSTR = windows_core::w!("S2DCacheDesiredState");
pub const CLUSTER_S2D_CACHE_FLASH_RESERVE_PERCENT: windows_core::PCWSTR = windows_core::w!("S2DCacheFlashReservePercent");
pub const CLUSTER_S2D_CACHE_METADATA_RESERVE: windows_core::PCWSTR = windows_core::w!("S2DCacheMetadataReserveBytes");
pub const CLUSTER_S2D_CACHE_PAGE_SIZE_KBYTES: windows_core::PCWSTR = windows_core::w!("S2DCachePageSizeKBytes");
pub const CLUSTER_S2D_ENABLED: windows_core::PCWSTR = windows_core::w!("S2DEnabled");
pub const CLUSTER_S2D_IO_LATENCY_THRESHOLD: windows_core::PCWSTR = windows_core::w!("S2DIOLatencyThreshold");
pub const CLUSTER_S2D_OPTIMIZATIONS: windows_core::PCWSTR = windows_core::w!("S2DOptimizations");
pub const CLUSTER_SET_ACCESS_TYPE_ALLOWED: u32 = 0u32;
pub const CLUSTER_SET_ACCESS_TYPE_DENIED: u32 = 1u32;
pub const CLUSTER_SHARED_VOLUMES_ROOT: windows_core::PCWSTR = windows_core::w!("SharedVolumesRoot");
pub const CLUSTER_SHARED_VOLUME_VSS_WRITER_OPERATION_TIMEOUT: windows_core::PCWSTR = windows_core::w!("SharedVolumeVssWriterOperationTimeout");
pub const CLUSTER_VERSION_FLAG_MIXED_MODE: u32 = 1u32;
pub const CLUSTER_VERSION_UNKNOWN: u32 = 4294967295u32;
pub const CLUSTER_WITNESS_DATABASE_WRITE_TIMEOUT: windows_core::PCWSTR = windows_core::w!("WitnessDatabaseWriteTimeout");
pub const CLUSTER_WITNESS_FAILED_RESTART_INTERVAL: windows_core::PCWSTR = windows_core::w!("WitnessRestartInterval");
pub const CLUS_ACCESS_ANY: u32 = 0u32;
pub const CLUS_ACCESS_READ: u32 = 1u32;
pub const CLUS_ACCESS_WRITE: u32 = 2u32;
pub const CLUS_AFFINITY_RULE_DIFFERENT_FAULT_DOMAIN: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(3i32);
pub const CLUS_AFFINITY_RULE_DIFFERENT_NODE: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(4i32);
pub const CLUS_AFFINITY_RULE_MAX: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(4i32);
pub const CLUS_AFFINITY_RULE_MIN: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(0i32);
pub const CLUS_AFFINITY_RULE_NONE: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(0i32);
pub const CLUS_AFFINITY_RULE_SAME_FAULT_DOMAIN: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(1i32);
pub const CLUS_AFFINITY_RULE_SAME_NODE: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(2i32);
pub const CLUS_CHAR_BROADCAST_DELETE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(32i32);
pub const CLUS_CHAR_CLONES: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(8192i32);
pub const CLUS_CHAR_COEXIST_IN_SHARED_VOLUME_GROUP: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(256i32);
pub const CLUS_CHAR_DELETE_REQUIRES_ALL_NODES: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(2i32);
pub const CLUS_CHAR_DRAIN_LOCAL_OFFLINE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(524288i32);
pub const CLUS_CHAR_INFRASTRUCTURE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(131072i32);
pub const CLUS_CHAR_LOCAL_QUORUM: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(4i32);
pub const CLUS_CHAR_LOCAL_QUORUM_DEBUG: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(8i32);
pub const CLUS_CHAR_MONITOR_DETACH: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(1024i32);
pub const CLUS_CHAR_MONITOR_REATTACH: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(2048i32);
pub const CLUS_CHAR_NOTIFY_NEW_OWNER: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(32768i32);
pub const CLUS_CHAR_NOT_PREEMPTABLE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(16384i32);
pub const CLUS_CHAR_OPERATION_CONTEXT: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(4096i32);
pub const CLUS_CHAR_PLACEMENT_DATA: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(512i32);
pub const CLUS_CHAR_QUORUM: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(1i32);
pub const CLUS_CHAR_REQUIRES_STATE_CHANGE_REASON: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(16i32);
pub const CLUS_CHAR_SINGLE_CLUSTER_INSTANCE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(64i32);
pub const CLUS_CHAR_SINGLE_GROUP_INSTANCE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(128i32);
pub const CLUS_CHAR_SUPPORTS_UNMONITORED_STATE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(65536i32);
pub const CLUS_CHAR_UNKNOWN: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(0i32);
pub const CLUS_CHAR_VETO_DRAIN: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(262144i32);
pub const CLUS_CREATE_CRYPT_CONTAINER_NOT_FOUND: u32 = 1u32;
pub const CLUS_FLAG_CORE: CLUS_FLAGS = CLUS_FLAGS(1i32);
pub const CLUS_GLOBAL: u32 = 1u32;
pub const CLUS_GROUP_DO_NOT_START: CLUS_GROUP_START_SETTING = CLUS_GROUP_START_SETTING(1i32);
pub const CLUS_GROUP_START_ALLOWED: CLUS_GROUP_START_SETTING = CLUS_GROUP_START_SETTING(2i32);
pub const CLUS_GROUP_START_ALWAYS: CLUS_GROUP_START_SETTING = CLUS_GROUP_START_SETTING(0i32);
pub const CLUS_GRP_MOVE_ALLOWED: u32 = 0u32;
pub const CLUS_GRP_MOVE_LOCKED: u32 = 1u32;
pub const CLUS_HYBRID_QUORUM: u32 = 1024u32;
pub const CLUS_MODIFY: u32 = 1u32;
pub const CLUS_NAME_RES_TYPE_CLUSTER_GROUPID: windows_core::PCWSTR = windows_core::w!("ClusterGroupId");
pub const CLUS_NAME_RES_TYPE_DATA_RESID: windows_core::PCWSTR = windows_core::w!("DataResourceId");
pub const CLUS_NAME_RES_TYPE_LOG_MULTIPLE: windows_core::PCWSTR = windows_core::w!("LogSizeMultiple");
pub const CLUS_NAME_RES_TYPE_LOG_RESID: windows_core::PCWSTR = windows_core::w!("LogResourceId");
pub const CLUS_NAME_RES_TYPE_LOG_VOLUME: windows_core::PCWSTR = windows_core::w!("LogVolume");
pub const CLUS_NAME_RES_TYPE_MINIMUM_LOG_SIZE: windows_core::PCWSTR = windows_core::w!("MinimumLogSizeInBytes");
pub const CLUS_NAME_RES_TYPE_REPLICATION_GROUPID: windows_core::PCWSTR = windows_core::w!("ReplicationGroupId");
pub const CLUS_NAME_RES_TYPE_REPLICATION_GROUP_TYPE: windows_core::PCWSTR = windows_core::w!("ReplicationClusterGroupType");
pub const CLUS_NAME_RES_TYPE_SOURCE_RESID: windows_core::PCWSTR = windows_core::w!("SourceResourceId");
pub const CLUS_NAME_RES_TYPE_SOURCE_VOLUMES: windows_core::PCWSTR = windows_core::w!("SourceVolumes");
pub const CLUS_NAME_RES_TYPE_TARGET_RESID: windows_core::PCWSTR = windows_core::w!("TargetResourceId");
pub const CLUS_NAME_RES_TYPE_TARGET_VOLUMES: windows_core::PCWSTR = windows_core::w!("TargetVolumes");
pub const CLUS_NAME_RES_TYPE_UNIT_LOG_SIZE_CHANGE: windows_core::PCWSTR = windows_core::w!("UnitOfLogSizeChangeInBytes");
pub const CLUS_NODE_MAJORITY_QUORUM: u32 = 0u32;
pub const CLUS_NOT_GLOBAL: u32 = 0u32;
pub const CLUS_NO_MODIFY: u32 = 0u32;
pub const CLUS_OBJECT_AFFINITYRULE: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(9i32);
pub const CLUS_OBJECT_CLUSTER: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(7i32);
pub const CLUS_OBJECT_GROUP: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(3i32);
pub const CLUS_OBJECT_GROUPSET: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(8i32);
pub const CLUS_OBJECT_INVALID: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(0i32);
pub const CLUS_OBJECT_NETINTERFACE: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(6i32);
pub const CLUS_OBJECT_NETWORK: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(5i32);
pub const CLUS_OBJECT_NODE: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(4i32);
pub const CLUS_OBJECT_RESOURCE: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(1i32);
pub const CLUS_OBJECT_RESOURCE_TYPE: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(2i32);
pub const CLUS_OBJECT_USER: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(128i32);
pub const CLUS_RESCLASS_NETWORK: CLUSTER_RESOURCE_CLASS = CLUSTER_RESOURCE_CLASS(2i32);
pub const CLUS_RESCLASS_STORAGE: CLUSTER_RESOURCE_CLASS = CLUSTER_RESOURCE_CLASS(1i32);
pub const CLUS_RESCLASS_UNKNOWN: CLUSTER_RESOURCE_CLASS = CLUSTER_RESOURCE_CLASS(0i32);
pub const CLUS_RESCLASS_USER: CLUSTER_RESOURCE_CLASS = CLUSTER_RESOURCE_CLASS(32768i32);
pub const CLUS_RESDLL_OFFLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 64u32;
pub const CLUS_RESDLL_OFFLINE_DUE_TO_EMBEDDED_FAILURE: u32 = 16u32;
pub const CLUS_RESDLL_OFFLINE_IGNORE_NETWORK_CONNECTIVITY: u32 = 32u32;
pub const CLUS_RESDLL_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUS_RESDLL_OFFLINE_QUEUE_ENABLED: u32 = 4u32;
pub const CLUS_RESDLL_OFFLINE_RETURNING_TO_SOURCE_NODE_BECAUSE_OF_ERROR: u32 = 8u32;
pub const CLUS_RESDLL_OFFLINE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 2u32;
pub const CLUS_RESDLL_ONLINE_IGNORE_NETWORK_CONNECTIVITY: u32 = 16u32;
pub const CLUS_RESDLL_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 2u32;
pub const CLUS_RESDLL_ONLINE_RECOVER_MONITOR_STATE: u32 = 1u32;
pub const CLUS_RESDLL_ONLINE_RESTORE_ONLINE_STATE: u32 = 8u32;
pub const CLUS_RESDLL_ONLINE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 4u32;
pub const CLUS_RESDLL_OPEN_DONT_DELETE_TEMP_DISK: u32 = 2u32;
pub const CLUS_RESDLL_OPEN_RECOVER_MONITOR_STATE: u32 = 1u32;
pub const CLUS_RESSUBCLASS_NETWORK_INTERNET_PROTOCOL: CLUS_RESSUBCLASS_NETWORK = CLUS_RESSUBCLASS_NETWORK(-2147483648i32);
pub const CLUS_RESSUBCLASS_SHARED: CLUS_RESSUBCLASS = CLUS_RESSUBCLASS(-2147483648i32);
pub const CLUS_RESSUBCLASS_STORAGE_DISK: CLUS_RESSUBCLASS_STORAGE = CLUS_RESSUBCLASS_STORAGE(1073741824i32);
pub const CLUS_RESSUBCLASS_STORAGE_REPLICATION: CLUS_RESSUBCLASS_STORAGE = CLUS_RESSUBCLASS_STORAGE(268435456i32);
pub const CLUS_RESSUBCLASS_STORAGE_SHARED_BUS: CLUS_RESSUBCLASS_STORAGE = CLUS_RESSUBCLASS_STORAGE(-2147483648i32);
pub const CLUS_RESTYPE_NAME_CAU: windows_core::PCWSTR = windows_core::w!("ClusterAwareUpdatingResource");
pub const CLUS_RESTYPE_NAME_CLOUD_WITNESS: windows_core::PCWSTR = windows_core::w!("Cloud Witness");
pub const CLUS_RESTYPE_NAME_CONTAINER: windows_core::PCWSTR = windows_core::w!("Container");
pub const CLUS_RESTYPE_NAME_CROSS_CLUSTER: windows_core::PCWSTR = windows_core::w!("Cross Cluster Dependency Orchestrator");
pub const CLUS_RESTYPE_NAME_DFS: windows_core::PCWSTR = windows_core::w!("Distributed File System");
pub const CLUS_RESTYPE_NAME_DFSR: windows_core::PCWSTR = windows_core::w!("DFS Replicated Folder");
pub const CLUS_RESTYPE_NAME_DHCP: windows_core::PCWSTR = windows_core::w!("DHCP Service");
pub const CLUS_RESTYPE_NAME_DNN: windows_core::PCWSTR = windows_core::w!("Distributed Network Name");
pub const CLUS_RESTYPE_NAME_FILESERVER: windows_core::PCWSTR = windows_core::w!("File Server");
pub const CLUS_RESTYPE_NAME_FILESHR: windows_core::PCWSTR = windows_core::w!("File Share");
pub const CLUS_RESTYPE_NAME_FSWITNESS: windows_core::PCWSTR = windows_core::w!("File Share Witness");
pub const CLUS_RESTYPE_NAME_GENAPP: windows_core::PCWSTR = windows_core::w!("Generic Application");
pub const CLUS_RESTYPE_NAME_GENSCRIPT: windows_core::PCWSTR = windows_core::w!("Generic Script");
pub const CLUS_RESTYPE_NAME_GENSVC: windows_core::PCWSTR = windows_core::w!("Generic Service");
pub const CLUS_RESTYPE_NAME_HARDDISK: windows_core::PCWSTR = windows_core::w!("Physical Disk");
pub const CLUS_RESTYPE_NAME_HCSVM: windows_core::PCWSTR = windows_core::w!("HCS Virtual Machine");
pub const CLUS_RESTYPE_NAME_HEALTH_SERVICE: windows_core::PCWSTR = windows_core::w!("Health Service");
pub const CLUS_RESTYPE_NAME_IPADDR: windows_core::PCWSTR = windows_core::w!("IP Address");
pub const CLUS_RESTYPE_NAME_IPV6_NATIVE: windows_core::PCWSTR = windows_core::w!("IPv6 Address");
pub const CLUS_RESTYPE_NAME_IPV6_TUNNEL: windows_core::PCWSTR = windows_core::w!("IPv6 Tunnel Address");
pub const CLUS_RESTYPE_NAME_ISCSITARGET: windows_core::PCWSTR = windows_core::w!("iSCSI Target Server");
pub const CLUS_RESTYPE_NAME_ISNS: windows_core::PCWSTR = windows_core::w!("Microsoft iSNS");
pub const CLUS_RESTYPE_NAME_MSDTC: windows_core::PCWSTR = windows_core::w!("Distributed Transaction Coordinator");
pub const CLUS_RESTYPE_NAME_MSMQ: windows_core::PCWSTR = windows_core::w!("Microsoft Message Queue Server");
pub const CLUS_RESTYPE_NAME_MSMQ_TRIGGER: windows_core::PCWSTR = windows_core::w!("MSMQTriggers");
pub const CLUS_RESTYPE_NAME_NAT: windows_core::PCWSTR = windows_core::w!("Nat");
pub const CLUS_RESTYPE_NAME_NETNAME: windows_core::PCWSTR = windows_core::w!("Network Name");
pub const CLUS_RESTYPE_NAME_NETWORK_FILE_SYSTEM: windows_core::PCWSTR = windows_core::w!("Network File System");
pub const CLUS_RESTYPE_NAME_NEW_MSMQ: windows_core::PCWSTR = windows_core::w!("MSMQ");
pub const CLUS_RESTYPE_NAME_NFS: windows_core::PCWSTR = windows_core::w!("NFS Share");
pub const CLUS_RESTYPE_NAME_NFS_MSNS: windows_core::PCWSTR = windows_core::w!("NFS Multi Server Namespace");
pub const CLUS_RESTYPE_NAME_NFS_V2: windows_core::PCWSTR = windows_core::w!("Network File System");
pub const CLUS_RESTYPE_NAME_NV_PROVIDER_ADDRESS: windows_core::PCWSTR = windows_core::w!("Provider Address");
pub const CLUS_RESTYPE_NAME_PHYS_DISK: windows_core::PCWSTR = windows_core::w!("Physical Disk");
pub const CLUS_RESTYPE_NAME_PRTSPLR: windows_core::PCWSTR = windows_core::w!("Print Spooler");
pub const CLUS_RESTYPE_NAME_SCALEOUT_MASTER: windows_core::PCWSTR = windows_core::w!("Scaleout Master");
pub const CLUS_RESTYPE_NAME_SCALEOUT_WORKER: windows_core::PCWSTR = windows_core::w!("Scaleout Worker");
pub const CLUS_RESTYPE_NAME_SDDC_MANAGEMENT: windows_core::PCWSTR = windows_core::w!("SDDC Management");
pub const CLUS_RESTYPE_NAME_SODAFILESERVER: windows_core::PCWSTR = windows_core::w!("Scale Out File Server");
pub const CLUS_RESTYPE_NAME_STORAGE_POLICIES: windows_core::PCWSTR = windows_core::w!("Storage Policies");
pub const CLUS_RESTYPE_NAME_STORAGE_POOL: windows_core::PCWSTR = windows_core::w!("Storage Pool");
pub const CLUS_RESTYPE_NAME_STORAGE_REPLICA: windows_core::PCWSTR = windows_core::w!("Storage Replica");
pub const CLUS_RESTYPE_NAME_STORQOS: windows_core::PCWSTR = windows_core::w!("Storage QoS Policy Manager");
pub const CLUS_RESTYPE_NAME_TASKSCHEDULER: windows_core::PCWSTR = windows_core::w!("Task Scheduler");
pub const CLUS_RESTYPE_NAME_VIRTUAL_IPV4: windows_core::PCWSTR = windows_core::w!("Disjoint IPv4 Address");
pub const CLUS_RESTYPE_NAME_VIRTUAL_IPV6: windows_core::PCWSTR = windows_core::w!("Disjoint IPv6 Address");
pub const CLUS_RESTYPE_NAME_VM: windows_core::PCWSTR = windows_core::w!("Virtual Machine");
pub const CLUS_RESTYPE_NAME_VMREPLICA_BROKER: windows_core::PCWSTR = windows_core::w!("Virtual Machine Replication Broker");
pub const CLUS_RESTYPE_NAME_VMREPLICA_COORDINATOR: windows_core::PCWSTR = windows_core::w!("Virtual Machine Replication Coordinator");
pub const CLUS_RESTYPE_NAME_VM_CONFIG: windows_core::PCWSTR = windows_core::w!("Virtual Machine Configuration");
pub const CLUS_RESTYPE_NAME_VM_WMI: windows_core::PCWSTR = windows_core::w!("Virtual Machine Cluster WMI");
pub const CLUS_RESTYPE_NAME_VSSTASK: windows_core::PCWSTR = windows_core::w!("Volume Shadow Copy Service Task");
pub const CLUS_RESTYPE_NAME_WINS: windows_core::PCWSTR = windows_core::w!("WINS Service");
pub const CLUS_RES_NAME_SCALEOUT_MASTER: windows_core::PCWSTR = windows_core::w!("Scaleout Master");
pub const CLUS_RES_NAME_SCALEOUT_WORKER: windows_core::PCWSTR = windows_core::w!("Scaleout Worker");
pub const CREATEDC_PRESENT: u32 = 2u32;
pub const CREATE_CLUSTER_MAJOR_VERSION_MASK: u32 = 4294967040u32;
pub const CREATE_CLUSTER_VERSION: u32 = 1536u32;
pub const CTCTL_GET_FAULT_DOMAIN_STATE: CLCTL_CODES = CLCTL_CODES(789i32);
pub const CTCTL_GET_ROUTESTATUS_BASIC: CLCTL_CODES = CLCTL_CODES(781i32);
pub const CTCTL_GET_ROUTESTATUS_EXTENDED: CLCTL_CODES = CLCTL_CODES(785i32);
pub const ClusGroupTypeAvailableStorage: CLUSGROUP_TYPE = CLUSGROUP_TYPE(2i32);
pub const ClusGroupTypeClusterUpdateAgent: CLUSGROUP_TYPE = CLUSGROUP_TYPE(117i32);
pub const ClusGroupTypeCoreCluster: CLUSGROUP_TYPE = CLUSGROUP_TYPE(1i32);
pub const ClusGroupTypeCoreSddc: CLUSGROUP_TYPE = CLUSGROUP_TYPE(123i32);
pub const ClusGroupTypeCrossClusterOrchestrator: CLUSGROUP_TYPE = CLUSGROUP_TYPE(121i32);
pub const ClusGroupTypeDhcpServer: CLUSGROUP_TYPE = CLUSGROUP_TYPE(102i32);
pub const ClusGroupTypeDtc: CLUSGROUP_TYPE = CLUSGROUP_TYPE(103i32);
pub const ClusGroupTypeFileServer: CLUSGROUP_TYPE = CLUSGROUP_TYPE(100i32);
pub const ClusGroupTypeGenericApplication: CLUSGROUP_TYPE = CLUSGROUP_TYPE(107i32);
pub const ClusGroupTypeGenericScript: CLUSGROUP_TYPE = CLUSGROUP_TYPE(109i32);
pub const ClusGroupTypeGenericService: CLUSGROUP_TYPE = CLUSGROUP_TYPE(108i32);
pub const ClusGroupTypeIScsiNameService: CLUSGROUP_TYPE = CLUSGROUP_TYPE(110i32);
pub const ClusGroupTypeIScsiTarget: CLUSGROUP_TYPE = CLUSGROUP_TYPE(113i32);
pub const ClusGroupTypeInfrastructureFileServer: CLUSGROUP_TYPE = CLUSGROUP_TYPE(122i32);
pub const ClusGroupTypeMsmq: CLUSGROUP_TYPE = CLUSGROUP_TYPE(104i32);
pub const ClusGroupTypePrintServer: CLUSGROUP_TYPE = CLUSGROUP_TYPE(101i32);
pub const ClusGroupTypeScaleoutCluster: CLUSGROUP_TYPE = CLUSGROUP_TYPE(118i32);
pub const ClusGroupTypeScaleoutFileServer: CLUSGROUP_TYPE = CLUSGROUP_TYPE(114i32);
pub const ClusGroupTypeSharedVolume: CLUSGROUP_TYPE = CLUSGROUP_TYPE(4i32);
pub const ClusGroupTypeStandAloneDfs: CLUSGROUP_TYPE = CLUSGROUP_TYPE(106i32);
pub const ClusGroupTypeStoragePool: CLUSGROUP_TYPE = CLUSGROUP_TYPE(5i32);
pub const ClusGroupTypeStorageReplica: CLUSGROUP_TYPE = CLUSGROUP_TYPE(119i32);
pub const ClusGroupTypeTaskScheduler: CLUSGROUP_TYPE = CLUSGROUP_TYPE(116i32);
pub const ClusGroupTypeTemporary: CLUSGROUP_TYPE = CLUSGROUP_TYPE(3i32);
pub const ClusGroupTypeTsSessionBroker: CLUSGROUP_TYPE = CLUSGROUP_TYPE(112i32);
pub const ClusGroupTypeUnknown: CLUSGROUP_TYPE = CLUSGROUP_TYPE(9999i32);
pub const ClusGroupTypeVMReplicaBroker: CLUSGROUP_TYPE = CLUSGROUP_TYPE(115i32);
pub const ClusGroupTypeVMReplicaCoordinator: CLUSGROUP_TYPE = CLUSGROUP_TYPE(120i32);
pub const ClusGroupTypeVirtualMachine: CLUSGROUP_TYPE = CLUSGROUP_TYPE(111i32);
pub const ClusGroupTypeWins: CLUSGROUP_TYPE = CLUSGROUP_TYPE(105i32);
pub const ClusterGroupAllowFailback: CLUSTER_GROUP_AUTOFAILBACK_TYPE = CLUSTER_GROUP_AUTOFAILBACK_TYPE(1i32);
pub const ClusterGroupFailbackTypeCount: CLUSTER_GROUP_AUTOFAILBACK_TYPE = CLUSTER_GROUP_AUTOFAILBACK_TYPE(2i32);
pub const ClusterGroupFailed: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(2i32);
pub const ClusterGroupOffline: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(1i32);
pub const ClusterGroupOnline: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(0i32);
pub const ClusterGroupPartialOnline: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(3i32);
pub const ClusterGroupPending: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(4i32);
pub const ClusterGroupPreventFailback: CLUSTER_GROUP_AUTOFAILBACK_TYPE = CLUSTER_GROUP_AUTOFAILBACK_TYPE(0i32);
pub const ClusterGroupStateUnknown: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(-1i32);
pub const ClusterNetInterfaceFailed: CLUSTER_NETINTERFACE_STATE = CLUSTER_NETINTERFACE_STATE(1i32);
pub const ClusterNetInterfaceStateUnknown: CLUSTER_NETINTERFACE_STATE = CLUSTER_NETINTERFACE_STATE(-1i32);
pub const ClusterNetInterfaceUnavailable: CLUSTER_NETINTERFACE_STATE = CLUSTER_NETINTERFACE_STATE(0i32);
pub const ClusterNetInterfaceUnreachable: CLUSTER_NETINTERFACE_STATE = CLUSTER_NETINTERFACE_STATE(2i32);
pub const ClusterNetInterfaceUp: CLUSTER_NETINTERFACE_STATE = CLUSTER_NETINTERFACE_STATE(3i32);
pub const ClusterNetworkDown: CLUSTER_NETWORK_STATE = CLUSTER_NETWORK_STATE(1i32);
pub const ClusterNetworkPartitioned: CLUSTER_NETWORK_STATE = CLUSTER_NETWORK_STATE(2i32);
pub const ClusterNetworkRoleClientAccess: CLUSTER_NETWORK_ROLE = CLUSTER_NETWORK_ROLE(2i32);
pub const ClusterNetworkRoleInternalAndClient: CLUSTER_NETWORK_ROLE = CLUSTER_NETWORK_ROLE(3i32);
pub const ClusterNetworkRoleInternalUse: CLUSTER_NETWORK_ROLE = CLUSTER_NETWORK_ROLE(1i32);
pub const ClusterNetworkRoleNone: CLUSTER_NETWORK_ROLE = CLUSTER_NETWORK_ROLE(0i32);
pub const ClusterNetworkStateUnknown: CLUSTER_NETWORK_STATE = CLUSTER_NETWORK_STATE(-1i32);
pub const ClusterNetworkUnavailable: CLUSTER_NETWORK_STATE = CLUSTER_NETWORK_STATE(0i32);
pub const ClusterNetworkUp: CLUSTER_NETWORK_STATE = CLUSTER_NETWORK_STATE(3i32);
pub const ClusterNodeDown: CLUSTER_NODE_STATE = CLUSTER_NODE_STATE(1i32);
pub const ClusterNodeDrainStatusCount: CLUSTER_NODE_DRAIN_STATUS = CLUSTER_NODE_DRAIN_STATUS(4i32);
pub const ClusterNodeJoining: CLUSTER_NODE_STATE = CLUSTER_NODE_STATE(3i32);
pub const ClusterNodePaused: CLUSTER_NODE_STATE = CLUSTER_NODE_STATE(2i32);
pub const ClusterNodeResumeFailbackTypeCount: CLUSTER_NODE_RESUME_FAILBACK_TYPE = CLUSTER_NODE_RESUME_FAILBACK_TYPE(3i32);
pub const ClusterNodeStateUnknown: CLUSTER_NODE_STATE = CLUSTER_NODE_STATE(-1i32);
pub const ClusterNodeUp: CLUSTER_NODE_STATE = CLUSTER_NODE_STATE(0i32);
pub const ClusterResourceApplicationOSHeartBeat: CLUSTER_RESOURCE_APPLICATION_STATE = CLUSTER_RESOURCE_APPLICATION_STATE(2i32);
pub const ClusterResourceApplicationReady: CLUSTER_RESOURCE_APPLICATION_STATE = CLUSTER_RESOURCE_APPLICATION_STATE(3i32);
pub const ClusterResourceApplicationStateUnknown: CLUSTER_RESOURCE_APPLICATION_STATE = CLUSTER_RESOURCE_APPLICATION_STATE(1i32);
pub const ClusterResourceDontRestart: CLUSTER_RESOURCE_RESTART_ACTION = CLUSTER_RESOURCE_RESTART_ACTION(0i32);
pub const ClusterResourceEmbeddedFailureActionLogOnly: CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION(1i32);
pub const ClusterResourceEmbeddedFailureActionNone: CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION(0i32);
pub const ClusterResourceEmbeddedFailureActionRecover: CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION(2i32);
pub const ClusterResourceFailed: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(4i32);
pub const ClusterResourceInherited: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(0i32);
pub const ClusterResourceInitializing: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(1i32);
pub const ClusterResourceOffline: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(3i32);
pub const ClusterResourceOfflinePending: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(130i32);
pub const ClusterResourceOnline: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(2i32);
pub const ClusterResourceOnlinePending: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(129i32);
pub const ClusterResourcePending: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(128i32);
pub const ClusterResourceRestartActionCount: CLUSTER_RESOURCE_RESTART_ACTION = CLUSTER_RESOURCE_RESTART_ACTION(3i32);
pub const ClusterResourceRestartNoNotify: CLUSTER_RESOURCE_RESTART_ACTION = CLUSTER_RESOURCE_RESTART_ACTION(1i32);
pub const ClusterResourceRestartNotify: CLUSTER_RESOURCE_RESTART_ACTION = CLUSTER_RESOURCE_RESTART_ACTION(2i32);
pub const ClusterResourceStateUnknown: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(-1i32);
pub const ClusterRoleClustered: CLUSTER_ROLE_STATE = CLUSTER_ROLE_STATE(0i32);
pub const ClusterRoleDFSReplicatedFolder: CLUSTER_ROLE = CLUSTER_ROLE(15i32);
pub const ClusterRoleDHCP: CLUSTER_ROLE = CLUSTER_ROLE(0i32);
pub const ClusterRoleDTC: CLUSTER_ROLE = CLUSTER_ROLE(1i32);
pub const ClusterRoleDistributedFileSystem: CLUSTER_ROLE = CLUSTER_ROLE(16i32);
pub const ClusterRoleDistributedNetworkName: CLUSTER_ROLE = CLUSTER_ROLE(17i32);
pub const ClusterRoleFileServer: CLUSTER_ROLE = CLUSTER_ROLE(2i32);
pub const ClusterRoleFileShare: CLUSTER_ROLE = CLUSTER_ROLE(18i32);
pub const ClusterRoleFileShareWitness: CLUSTER_ROLE = CLUSTER_ROLE(19i32);
pub const ClusterRoleGenericApplication: CLUSTER_ROLE = CLUSTER_ROLE(3i32);
pub const ClusterRoleGenericScript: CLUSTER_ROLE = CLUSTER_ROLE(4i32);
pub const ClusterRoleGenericService: CLUSTER_ROLE = CLUSTER_ROLE(5i32);
pub const ClusterRoleHardDisk: CLUSTER_ROLE = CLUSTER_ROLE(20i32);
pub const ClusterRoleIPAddress: CLUSTER_ROLE = CLUSTER_ROLE(21i32);
pub const ClusterRoleIPV6Address: CLUSTER_ROLE = CLUSTER_ROLE(22i32);
pub const ClusterRoleIPV6TunnelAddress: CLUSTER_ROLE = CLUSTER_ROLE(23i32);
pub const ClusterRoleISCSINameServer: CLUSTER_ROLE = CLUSTER_ROLE(6i32);
pub const ClusterRoleISCSITargetServer: CLUSTER_ROLE = CLUSTER_ROLE(24i32);
pub const ClusterRoleMSMQ: CLUSTER_ROLE = CLUSTER_ROLE(7i32);
pub const ClusterRoleNFS: CLUSTER_ROLE = CLUSTER_ROLE(8i32);
pub const ClusterRoleNetworkFileSystem: CLUSTER_ROLE = CLUSTER_ROLE(14i32);
pub const ClusterRoleNetworkName: CLUSTER_ROLE = CLUSTER_ROLE(25i32);
pub const ClusterRolePhysicalDisk: CLUSTER_ROLE = CLUSTER_ROLE(26i32);
pub const ClusterRolePrintServer: CLUSTER_ROLE = CLUSTER_ROLE(9i32);
pub const ClusterRoleSODAFileServer: CLUSTER_ROLE = CLUSTER_ROLE(27i32);
pub const ClusterRoleStandAloneNamespaceServer: CLUSTER_ROLE = CLUSTER_ROLE(10i32);
pub const ClusterRoleStoragePool: CLUSTER_ROLE = CLUSTER_ROLE(28i32);
pub const ClusterRoleTaskScheduler: CLUSTER_ROLE = CLUSTER_ROLE(13i32);
pub const ClusterRoleUnclustered: CLUSTER_ROLE_STATE = CLUSTER_ROLE_STATE(1i32);
pub const ClusterRoleUnknown: CLUSTER_ROLE_STATE = CLUSTER_ROLE_STATE(-1i32);
pub const ClusterRoleVirtualMachine: CLUSTER_ROLE = CLUSTER_ROLE(29i32);
pub const ClusterRoleVirtualMachineConfiguration: CLUSTER_ROLE = CLUSTER_ROLE(30i32);
pub const ClusterRoleVirtualMachineReplicaBroker: CLUSTER_ROLE = CLUSTER_ROLE(31i32);
pub const ClusterRoleVolumeShadowCopyServiceTask: CLUSTER_ROLE = CLUSTER_ROLE(11i32);
pub const ClusterRoleWINS: CLUSTER_ROLE = CLUSTER_ROLE(12i32);
pub const ClusterSetupPhaseAddClusterProperties: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(201i32);
pub const ClusterSetupPhaseAddNodeToCluster: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(301i32);
pub const ClusterSetupPhaseCleanupCOs: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(402i32);
pub const ClusterSetupPhaseCleanupNode: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(405i32);
pub const ClusterSetupPhaseClusterGroupOnline: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(206i32);
pub const ClusterSetupPhaseConfigureClusSvc: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(104i32);
pub const ClusterSetupPhaseConfigureClusterAccount: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(109i32);
pub const ClusterSetupPhaseContinue: CLUSTER_SETUP_PHASE_TYPE = CLUSTER_SETUP_PHASE_TYPE(2i32);
pub const ClusterSetupPhaseCoreGroupCleanup: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(406i32);
pub const ClusterSetupPhaseCreateClusterAccount: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(108i32);
pub const ClusterSetupPhaseCreateGroups: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(203i32);
pub const ClusterSetupPhaseCreateIPAddressResources: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(204i32);
pub const ClusterSetupPhaseCreateNetworkName: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(205i32);
pub const ClusterSetupPhaseCreateResourceTypes: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(202i32);
pub const ClusterSetupPhaseDeleteGroup: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(401i32);
pub const ClusterSetupPhaseEnd: CLUSTER_SETUP_PHASE_TYPE = CLUSTER_SETUP_PHASE_TYPE(3i32);
pub const ClusterSetupPhaseEvictNode: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(404i32);
pub const ClusterSetupPhaseFailureCleanup: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(999i32);
pub const ClusterSetupPhaseFatal: CLUSTER_SETUP_PHASE_SEVERITY = CLUSTER_SETUP_PHASE_SEVERITY(3i32);
pub const ClusterSetupPhaseFormingCluster: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(200i32);
pub const ClusterSetupPhaseGettingCurrentMembership: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(300i32);
pub const ClusterSetupPhaseInformational: CLUSTER_SETUP_PHASE_SEVERITY = CLUSTER_SETUP_PHASE_SEVERITY(1i32);
pub const ClusterSetupPhaseInitialize: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(1i32);
pub const ClusterSetupPhaseMoveGroup: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(400i32);
pub const ClusterSetupPhaseNodeUp: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(302i32);
pub const ClusterSetupPhaseOfflineGroup: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(403i32);
pub const ClusterSetupPhaseQueryClusterNameAccount: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(106i32);
pub const ClusterSetupPhaseReport: CLUSTER_SETUP_PHASE_TYPE = CLUSTER_SETUP_PHASE_TYPE(4i32);
pub const ClusterSetupPhaseStart: CLUSTER_SETUP_PHASE_TYPE = CLUSTER_SETUP_PHASE_TYPE(1i32);
pub const ClusterSetupPhaseStartingClusSvc: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(105i32);
pub const ClusterSetupPhaseValidateClusDisk: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(103i32);
pub const ClusterSetupPhaseValidateClusterNameAccount: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(107i32);
pub const ClusterSetupPhaseValidateNetft: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(102i32);
pub const ClusterSetupPhaseValidateNodeState: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(100i32);
pub const ClusterSetupPhaseWarning: CLUSTER_SETUP_PHASE_SEVERITY = CLUSTER_SETUP_PHASE_SEVERITY(2i32);
pub const ClusterSharedVolumeHWSnapshotCompleted: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE(2i32);
pub const ClusterSharedVolumePrepareForFreeze: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE(3i32);
pub const ClusterSharedVolumePrepareForHWSnapshot: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE(1i32);
pub const ClusterSharedVolumeRenameInputTypeNone: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(0i32);
pub const ClusterSharedVolumeRenameInputTypeVolumeGuid: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(4i32);
pub const ClusterSharedVolumeRenameInputTypeVolumeId: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(2i32);
pub const ClusterSharedVolumeRenameInputTypeVolumeName: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(3i32);
pub const ClusterSharedVolumeRenameInputTypeVolumeOffset: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(1i32);
pub const ClusterSharedVolumeSnapshotStateUnknown: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE(0i32);
pub const ClusterStateNotConfigured: NODE_CLUSTER_STATE = NODE_CLUSTER_STATE(1i32);
pub const ClusterStateNotInstalled: NODE_CLUSTER_STATE = NODE_CLUSTER_STATE(0i32);
pub const ClusterStateNotRunning: NODE_CLUSTER_STATE = NODE_CLUSTER_STATE(3i32);
pub const ClusterStateRunning: NODE_CLUSTER_STATE = NODE_CLUSTER_STATE(19i32);
pub const ClusterStorageNodeDown: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(2i32);
pub const ClusterStorageNodePaused: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(3i32);
pub const ClusterStorageNodeStarting: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(4i32);
pub const ClusterStorageNodeStateUnknown: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(0i32);
pub const ClusterStorageNodeStopping: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(5i32);
pub const ClusterStorageNodeUp: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(1i32);
pub const ClusterUpgradePhaseInitialize: CLUSTER_UPGRADE_PHASE = CLUSTER_UPGRADE_PHASE(1i32);
pub const ClusterUpgradePhaseInstallingNewComponents: CLUSTER_UPGRADE_PHASE = CLUSTER_UPGRADE_PHASE(4i32);
pub const ClusterUpgradePhaseUpgradeComplete: CLUSTER_UPGRADE_PHASE = CLUSTER_UPGRADE_PHASE(5i32);
pub const ClusterUpgradePhaseUpgradingComponents: CLUSTER_UPGRADE_PHASE = CLUSTER_UPGRADE_PHASE(3i32);
pub const ClusterUpgradePhaseValidatingUpgrade: CLUSTER_UPGRADE_PHASE = CLUSTER_UPGRADE_PHASE(2i32);
pub const DNS_LENGTH: u32 = 64u32;
pub const DoNotFailbackGroups: CLUSTER_NODE_RESUME_FAILBACK_TYPE = CLUSTER_NODE_RESUME_FAILBACK_TYPE(0i32);
pub const ENABLE_CLUSTER_SHARED_VOLUMES: windows_core::PCWSTR = windows_core::w!("EnableSharedVolumes");
pub const FAILURE_TYPE_EMBEDDED: FAILURE_TYPE = FAILURE_TYPE(1i32);
pub const FAILURE_TYPE_GENERAL: FAILURE_TYPE = FAILURE_TYPE(0i32);
pub const FAILURE_TYPE_NETWORK_LOSS: FAILURE_TYPE = FAILURE_TYPE(2i32);
pub const FE_UPGRADE_VERSION: u32 = 4u32;
pub const FILESHARE_CHANGE_ADD: FILESHARE_CHANGE_ENUM = FILESHARE_CHANGE_ENUM(1i32);
pub const FILESHARE_CHANGE_DEL: FILESHARE_CHANGE_ENUM = FILESHARE_CHANGE_ENUM(2i32);
pub const FILESHARE_CHANGE_MODIFY: FILESHARE_CHANGE_ENUM = FILESHARE_CHANGE_ENUM(3i32);
pub const FILESHARE_CHANGE_NONE: FILESHARE_CHANGE_ENUM = FILESHARE_CHANGE_ENUM(0i32);
pub const FailbackGroupsImmediately: CLUSTER_NODE_RESUME_FAILBACK_TYPE = CLUSTER_NODE_RESUME_FAILBACK_TYPE(1i32);
pub const FailbackGroupsPerPolicy: CLUSTER_NODE_RESUME_FAILBACK_TYPE = CLUSTER_NODE_RESUME_FAILBACK_TYPE(2i32);
pub const GROUPSET_READY_SETTING_APPLICATION_READY: u32 = 4u32;
pub const GROUPSET_READY_SETTING_DELAY: u32 = 1u32;
pub const GROUPSET_READY_SETTING_ONLINE: u32 = 2u32;
pub const GROUPSET_READY_SETTING_OS_HEARTBEAT: u32 = 3u32;
pub const GROUP_FAILURE_INFO_VERSION_1: u32 = 1u32;
pub const GRP_PLACEMENT_OPTIONS_ALL: GRP_PLACEMENT_OPTIONS = GRP_PLACEMENT_OPTIONS(1i32);
pub const GRP_PLACEMENT_OPTIONS_DEFAULT: GRP_PLACEMENT_OPTIONS = GRP_PLACEMENT_OPTIONS(0i32);
pub const GRP_PLACEMENT_OPTIONS_DISABLE_AUTOBALANCING: GRP_PLACEMENT_OPTIONS = GRP_PLACEMENT_OPTIONS(1i32);
pub const GRP_PLACEMENT_OPTIONS_MIN_VALUE: GRP_PLACEMENT_OPTIONS = GRP_PLACEMENT_OPTIONS(0i32);
pub const GUID_PRESENT: u32 = 1u32;
pub const HCI_UPGRADE_BIT: u32 = 32768u32;
pub const LOCKED_MODE_FLAGS_DONT_REMOVE_FROM_MOVE_QUEUE: u32 = 1u32;
pub const LOG_ERROR: LOG_LEVEL = LOG_LEVEL(2i32);
pub const LOG_INFORMATION: LOG_LEVEL = LOG_LEVEL(0i32);
pub const LOG_SEVERE: LOG_LEVEL = LOG_LEVEL(3i32);
pub const LOG_WARNING: LOG_LEVEL = LOG_LEVEL(1i32);
pub const MAINTENANCE_MODE_V2_SIG: u32 = 2881155087u32;
pub const MAX_CLUSTERNAME_LENGTH: u32 = 63u32;
pub const MAX_CO_PASSWORD_LENGTH: u32 = 16u32;
pub const MAX_CO_PASSWORD_LENGTHEX: u32 = 127u32;
pub const MAX_CO_PASSWORD_STORAGEEX: u32 = 128u32;
pub const MAX_CREATINGDC_LENGTH: u32 = 256u32;
pub const MAX_OBJECTID: u32 = 64u32;
pub const MINIMUM_NEVER_PREEMPT_PRIORITY: windows_core::PCWSTR = windows_core::w!("MinimumNeverPreemptPriority");
pub const MINIMUM_PREEMPTOR_PRIORITY: windows_core::PCWSTR = windows_core::w!("MinimumPreemptorPriority");
pub const MN_UPGRADE_VERSION: u32 = 3u32;
pub const MaintenanceModeTypeDisableIsAliveCheck: MAINTENANCE_MODE_TYPE_ENUM = MAINTENANCE_MODE_TYPE_ENUM(1i32);
pub const MaintenanceModeTypeOfflineResource: MAINTENANCE_MODE_TYPE_ENUM = MAINTENANCE_MODE_TYPE_ENUM(2i32);
pub const MaintenanceModeTypeUnclusterResource: MAINTENANCE_MODE_TYPE_ENUM = MAINTENANCE_MODE_TYPE_ENUM(3i32);
pub const ModifyQuorum: CLUSTER_QUORUM_TYPE = CLUSTER_QUORUM_TYPE(1i32);
pub const NINETEEN_H1_UPGRADE_VERSION: u32 = 1u32;
pub const NINETEEN_H2_UPGRADE_VERSION: u32 = 2u32;
pub const NI_UPGRADE_VERSION: u32 = 2u32;
pub const NNLEN: u32 = 80u32;
pub const NT10_MAJOR_VERSION: u32 = 9u32;
pub const NT11_MAJOR_VERSION: u32 = 10u32;
pub const NT12_MAJOR_VERSION: u32 = 11u32;
pub const NT13_MAJOR_VERSION: u32 = 12u32;
pub const NT4SP4_MAJOR_VERSION: u32 = 2u32;
pub const NT4_MAJOR_VERSION: u32 = 1u32;
pub const NT51_MAJOR_VERSION: u32 = 4u32;
pub const NT5_MAJOR_VERSION: u32 = 3u32;
pub const NT6_MAJOR_VERSION: u32 = 5u32;
pub const NT7_MAJOR_VERSION: u32 = 6u32;
pub const NT8_MAJOR_VERSION: u32 = 7u32;
pub const NT9_MAJOR_VERSION: u32 = 8u32;
pub const NodeDrainStatusCompleted: CLUSTER_NODE_DRAIN_STATUS = CLUSTER_NODE_DRAIN_STATUS(2i32);
pub const NodeDrainStatusFailed: CLUSTER_NODE_DRAIN_STATUS = CLUSTER_NODE_DRAIN_STATUS(3i32);
pub const NodeDrainStatusInProgress: CLUSTER_NODE_DRAIN_STATUS = CLUSTER_NODE_DRAIN_STATUS(1i32);
pub const NodeDrainStatusNotInitiated: CLUSTER_NODE_DRAIN_STATUS = CLUSTER_NODE_DRAIN_STATUS(0i32);
pub const NodeStatusAvoidPlacement: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(32i32);
pub const NodeStatusDrainCompleted: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(8i32);
pub const NodeStatusDrainFailed: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(16i32);
pub const NodeStatusDrainInProgress: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(4i32);
pub const NodeStatusIsolated: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(1i32);
pub const NodeStatusMax: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(51i32);
pub const NodeStatusNormal: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(0i32);
pub const NodeStatusQuarantined: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(2i32);
pub const OperationalQuorum: CLUSTER_QUORUM_TYPE = CLUSTER_QUORUM_TYPE(0i32);
pub const PLACEMENT_OPTIONS_ALL: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(1023i32);
pub const PLACEMENT_OPTIONS_AVAILABILITY_SET_DOMAIN_AFFINITY: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(512i32);
pub const PLACEMENT_OPTIONS_CONSIDER_OFFLINE_VMS: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(2i32);
pub const PLACEMENT_OPTIONS_DEFAULT_PLACEMENT_OPTIONS: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(0i32);
pub const PLACEMENT_OPTIONS_DISABLE_CSV_VM_DEPENDENCY: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(1i32);
pub const PLACEMENT_OPTIONS_DONT_RESUME_AVAILABILTY_SET_VMS_WITH_EXISTING_TEMP_DISK: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(128i32);
pub const PLACEMENT_OPTIONS_DONT_RESUME_VMS_WITH_EXISTING_TEMP_DISK: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(32i32);
pub const PLACEMENT_OPTIONS_DONT_USE_CPU: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(8i32);
pub const PLACEMENT_OPTIONS_DONT_USE_LOCAL_TEMP_DISK: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(16i32);
pub const PLACEMENT_OPTIONS_DONT_USE_MEMORY: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(4i32);
pub const PLACEMENT_OPTIONS_MIN_VALUE: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(0i32);
pub const PLACEMENT_OPTIONS_SAVE_AVAILABILTY_SET_VMS_WITH_LOCAL_DISK_ON_DRAIN_OVERWRITE: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(256i32);
pub const PLACEMENT_OPTIONS_SAVE_VMS_WITH_LOCAL_DISK_ON_DRAIN_OVERWRITE: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(64i32);
pub const PriorityDisabled: CLUSTER_GROUP_PRIORITY = CLUSTER_GROUP_PRIORITY(0i32);
pub const PriorityHigh: CLUSTER_GROUP_PRIORITY = CLUSTER_GROUP_PRIORITY(3000i32);
pub const PriorityLow: CLUSTER_GROUP_PRIORITY = CLUSTER_GROUP_PRIORITY(1000i32);
pub const PriorityMedium: CLUSTER_GROUP_PRIORITY = CLUSTER_GROUP_PRIORITY(2000i32);
pub const RESOURCE_FAILURE_INFO_VERSION_1: u32 = 1u32;
pub const RESTYPE_MONITOR_SHUTTING_DOWN_CLUSSVC_CRASH: u32 = 2u32;
pub const RESTYPE_MONITOR_SHUTTING_DOWN_NODE_STOP: u32 = 1u32;
pub const RESUTIL_PROPITEM_IN_MEMORY: u32 = 8u32;
pub const RESUTIL_PROPITEM_READ_ONLY: u32 = 1u32;
pub const RESUTIL_PROPITEM_REQUIRED: u32 = 2u32;
pub const RESUTIL_PROPITEM_SIGNED: u32 = 4u32;
pub const RS3_UPGRADE_VERSION: u32 = 1u32;
pub const RS4_UPGRADE_VERSION: u32 = 2u32;
pub const RS5_UPGRADE_VERSION: u32 = 3u32;
pub const RedirectedIOReasonBitLockerInitializing: u64 = 16u64;
pub const RedirectedIOReasonFileSystemTiering: u64 = 8u64;
pub const RedirectedIOReasonMax: u64 = 9223372036854775808u64;
pub const RedirectedIOReasonReFs: u64 = 32u64;
pub const RedirectedIOReasonUnsafeFileSystemFilter: u64 = 2u64;
pub const RedirectedIOReasonUnsafeVolumeFilter: u64 = 4u64;
pub const RedirectedIOReasonUserRequest: u64 = 1u64;
pub const ResdllContextOperationTypeDrain: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(1i32);
pub const ResdllContextOperationTypeDrainFailure: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(2i32);
pub const ResdllContextOperationTypeEmbeddedFailure: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(3i32);
pub const ResdllContextOperationTypeFailback: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(0i32);
pub const ResdllContextOperationTypeNetworkDisconnect: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(5i32);
pub const ResdllContextOperationTypeNetworkDisconnectMoveRetry: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(6i32);
pub const ResdllContextOperationTypePreemption: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(4i32);
pub const ResourceExitStateContinue: RESOURCE_EXIT_STATE = RESOURCE_EXIT_STATE(0i32);
pub const ResourceExitStateMax: RESOURCE_EXIT_STATE = RESOURCE_EXIT_STATE(2i32);
pub const ResourceExitStateTerminate: RESOURCE_EXIT_STATE = RESOURCE_EXIT_STATE(1i32);
pub const RmonArbitrateResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(10i32);
pub const RmonDeadlocked: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(15i32);
pub const RmonDeletingResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(7i32);
pub const RmonIdle: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(1i32);
pub const RmonInitializing: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(0i32);
pub const RmonInitializingResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(3i32);
pub const RmonIsAlivePoll: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(8i32);
pub const RmonLooksAlivePoll: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(9i32);
pub const RmonOfflineResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(5i32);
pub const RmonOnlineResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(4i32);
pub const RmonReleaseResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(11i32);
pub const RmonResourceControl: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(12i32);
pub const RmonResourceTypeControl: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(13i32);
pub const RmonShutdownResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(6i32);
pub const RmonStartingResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(2i32);
pub const RmonTerminateResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(14i32);
pub const SET_APPINSTANCE_CSV_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR: u32 = 1u32;
pub const SR_REPLICATED_PARTITION_DISALLOW_MULTINODE_IO: u32 = 1u32;
pub const STARTUP_EX_ROUTINE: windows_core::PCSTR = windows_core::s!("StartupEx");
pub const STARTUP_ROUTINE: windows_core::PCSTR = windows_core::s!("Startup");
pub const SharedVolumeStateActive: CLUSTER_SHARED_VOLUME_STATE = CLUSTER_SHARED_VOLUME_STATE(2i32);
pub const SharedVolumeStateActiveRedirected: CLUSTER_SHARED_VOLUME_STATE = CLUSTER_SHARED_VOLUME_STATE(3i32);
pub const SharedVolumeStateActiveVolumeRedirected: CLUSTER_SHARED_VOLUME_STATE = CLUSTER_SHARED_VOLUME_STATE(4i32);
pub const SharedVolumeStatePaused: CLUSTER_SHARED_VOLUME_STATE = CLUSTER_SHARED_VOLUME_STATE(1i32);
pub const SharedVolumeStateUnavailable: CLUSTER_SHARED_VOLUME_STATE = CLUSTER_SHARED_VOLUME_STATE(0i32);
pub const SrDiskReplicationEligibleAlreadyInReplication: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(9i32);
pub const SrDiskReplicationEligibleFileSystemNotSupported: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(8i32);
pub const SrDiskReplicationEligibleInSameSite: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(7i32);
pub const SrDiskReplicationEligibleInsufficientFreeSpace: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(5i32);
pub const SrDiskReplicationEligibleNone: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(0i32);
pub const SrDiskReplicationEligibleNotGpt: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(3i32);
pub const SrDiskReplicationEligibleNotInSameSite: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(6i32);
pub const SrDiskReplicationEligibleOffline: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(2i32);
pub const SrDiskReplicationEligibleOther: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(9999i32);
pub const SrDiskReplicationEligiblePartitionLayoutMismatch: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(4i32);
pub const SrDiskReplicationEligibleSameAsSpecifiedDisk: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(10i32);
pub const SrDiskReplicationEligibleYes: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(1i32);
pub const SrReplicatedDiskTypeDestination: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(3i32);
pub const SrReplicatedDiskTypeLogDestination: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(4i32);
pub const SrReplicatedDiskTypeLogNotInParthership: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(6i32);
pub const SrReplicatedDiskTypeLogSource: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(2i32);
pub const SrReplicatedDiskTypeNone: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(0i32);
pub const SrReplicatedDiskTypeNotInParthership: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(5i32);
pub const SrReplicatedDiskTypeOther: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(7i32);
pub const SrReplicatedDiskTypeSource: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(1i32);
pub const USE_CLIENT_ACCESS_NETWORKS_FOR_CSV: windows_core::PCWSTR = windows_core::w!("UseClientAccessNetworksForSharedVolumes");
pub const VmResdllContextLiveMigration: VM_RESDLL_CONTEXT = VM_RESDLL_CONTEXT(4i32);
pub const VmResdllContextSave: VM_RESDLL_CONTEXT = VM_RESDLL_CONTEXT(1i32);
pub const VmResdllContextShutdown: VM_RESDLL_CONTEXT = VM_RESDLL_CONTEXT(2i32);
pub const VmResdllContextShutdownForce: VM_RESDLL_CONTEXT = VM_RESDLL_CONTEXT(3i32);
pub const VmResdllContextTurnOff: VM_RESDLL_CONTEXT = VM_RESDLL_CONTEXT(0i32);
pub const VolumeBackupInProgress: CLUSTER_SHARED_VOLUME_BACKUP_STATE = CLUSTER_SHARED_VOLUME_BACKUP_STATE(1i32);
pub const VolumeBackupNone: CLUSTER_SHARED_VOLUME_BACKUP_STATE = CLUSTER_SHARED_VOLUME_BACKUP_STATE(0i32);
pub const VolumeRedirectedIOReasonMax: u64 = 9223372036854775808u64;
pub const VolumeRedirectedIOReasonNoDiskConnectivity: u64 = 1u64;
pub const VolumeRedirectedIOReasonStorageSpaceNotAttached: u64 = 2u64;
pub const VolumeRedirectedIOReasonVolumeReplicationEnabled: u64 = 4u64;
pub const VolumeStateDismounted: CLUSTER_CSV_VOLUME_FAULT_STATE = CLUSTER_CSV_VOLUME_FAULT_STATE(8i32);
pub const VolumeStateInMaintenance: CLUSTER_CSV_VOLUME_FAULT_STATE = CLUSTER_CSV_VOLUME_FAULT_STATE(4i32);
pub const VolumeStateNoAccess: CLUSTER_CSV_VOLUME_FAULT_STATE = CLUSTER_CSV_VOLUME_FAULT_STATE(2i32);
pub const VolumeStateNoDirectIO: CLUSTER_CSV_VOLUME_FAULT_STATE = CLUSTER_CSV_VOLUME_FAULT_STATE(1i32);
pub const VolumeStateNoFaults: CLUSTER_CSV_VOLUME_FAULT_STATE = CLUSTER_CSV_VOLUME_FAULT_STATE(0i32);
pub const WS2016_RTM_UPGRADE_VERSION: u32 = 8u32;
pub const WS2016_TP4_UPGRADE_VERSION: u32 = 6u32;
pub const WS2016_TP5_UPGRADE_VERSION: u32 = 7u32;
pub const eResourceStateChangeReasonFailedMove: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(3i32);
pub const eResourceStateChangeReasonFailover: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(2i32);
pub const eResourceStateChangeReasonMove: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(1i32);
pub const eResourceStateChangeReasonRundown: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(5i32);
pub const eResourceStateChangeReasonShutdown: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(4i32);
pub const eResourceStateChangeReasonUnknown: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLCTL_CODES(pub i32);
impl windows_core::TypeKind for CLCTL_CODES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLCTL_CODES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLCTL_CODES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUADMEX_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for CLUADMEX_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUADMEX_OBJECT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUADMEX_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSCTL_AFFINITYRULE_CODES(pub i32);
impl windows_core::TypeKind for CLUSCTL_AFFINITYRULE_CODES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSCTL_AFFINITYRULE_CODES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSCTL_AFFINITYRULE_CODES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSCTL_CLUSTER_CODES(pub i32);
impl windows_core::TypeKind for CLUSCTL_CLUSTER_CODES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSCTL_CLUSTER_CODES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSCTL_CLUSTER_CODES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSCTL_GROUPSET_CODES(pub i32);
impl windows_core::TypeKind for CLUSCTL_GROUPSET_CODES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSCTL_GROUPSET_CODES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSCTL_GROUPSET_CODES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSCTL_GROUP_CODES(pub i32);
impl windows_core::TypeKind for CLUSCTL_GROUP_CODES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSCTL_GROUP_CODES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSCTL_GROUP_CODES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSCTL_NETINTERFACE_CODES(pub i32);
impl windows_core::TypeKind for CLUSCTL_NETINTERFACE_CODES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSCTL_NETINTERFACE_CODES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSCTL_NETINTERFACE_CODES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSCTL_NETWORK_CODES(pub i32);
impl windows_core::TypeKind for CLUSCTL_NETWORK_CODES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSCTL_NETWORK_CODES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSCTL_NETWORK_CODES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSCTL_NODE_CODES(pub i32);
impl windows_core::TypeKind for CLUSCTL_NODE_CODES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSCTL_NODE_CODES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSCTL_NODE_CODES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSCTL_RESOURCE_CODES(pub i32);
impl windows_core::TypeKind for CLUSCTL_RESOURCE_CODES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSCTL_RESOURCE_CODES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSCTL_RESOURCE_CODES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSCTL_RESOURCE_TYPE_CODES(pub i32);
impl windows_core::TypeKind for CLUSCTL_RESOURCE_TYPE_CODES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSCTL_RESOURCE_TYPE_CODES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSCTL_RESOURCE_TYPE_CODES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSGROUP_TYPE(pub i32);
impl windows_core::TypeKind for CLUSGROUP_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSGROUP_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSGROUP_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSPROP_IPADDR_ENABLENETBIOS(pub i32);
impl windows_core::TypeKind for CLUSPROP_IPADDR_ENABLENETBIOS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSPROP_IPADDR_ENABLENETBIOS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSPROP_IPADDR_ENABLENETBIOS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSPROP_PIFLAGS(pub i32);
impl windows_core::TypeKind for CLUSPROP_PIFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSPROP_PIFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSPROP_PIFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTERSET_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for CLUSTERSET_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTERSET_OBJECT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTERSET_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CHANGE(pub i32);
impl windows_core::TypeKind for CLUSTER_CHANGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CHANGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CHANGE_CLUSTER_V2(pub i32);
impl windows_core::TypeKind for CLUSTER_CHANGE_CLUSTER_V2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CHANGE_CLUSTER_V2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_CLUSTER_V2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CHANGE_GROUPSET_V2(pub i32);
impl windows_core::TypeKind for CLUSTER_CHANGE_GROUPSET_V2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CHANGE_GROUPSET_V2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_GROUPSET_V2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CHANGE_GROUP_V2(pub i32);
impl windows_core::TypeKind for CLUSTER_CHANGE_GROUP_V2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CHANGE_GROUP_V2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_GROUP_V2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CHANGE_NETINTERFACE_V2(pub i32);
impl windows_core::TypeKind for CLUSTER_CHANGE_NETINTERFACE_V2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CHANGE_NETINTERFACE_V2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_NETINTERFACE_V2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CHANGE_NETWORK_V2(pub i32);
impl windows_core::TypeKind for CLUSTER_CHANGE_NETWORK_V2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CHANGE_NETWORK_V2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_NETWORK_V2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2(pub i32);
impl windows_core::TypeKind for CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CHANGE_NODE_V2(pub i32);
impl windows_core::TypeKind for CLUSTER_CHANGE_NODE_V2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CHANGE_NODE_V2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_NODE_V2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CHANGE_QUORUM_V2(pub i32);
impl windows_core::TypeKind for CLUSTER_CHANGE_QUORUM_V2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CHANGE_QUORUM_V2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_QUORUM_V2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CHANGE_REGISTRY_V2(pub i32);
impl windows_core::TypeKind for CLUSTER_CHANGE_REGISTRY_V2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CHANGE_REGISTRY_V2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_REGISTRY_V2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CHANGE_RESOURCE_TYPE_V2(pub i32);
impl windows_core::TypeKind for CLUSTER_CHANGE_RESOURCE_TYPE_V2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CHANGE_RESOURCE_TYPE_V2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_RESOURCE_TYPE_V2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CHANGE_RESOURCE_V2(pub i32);
impl windows_core::TypeKind for CLUSTER_CHANGE_RESOURCE_V2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CHANGE_RESOURCE_V2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_RESOURCE_V2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CHANGE_SHARED_VOLUME_V2(pub i32);
impl windows_core::TypeKind for CLUSTER_CHANGE_SHARED_VOLUME_V2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CHANGE_SHARED_VOLUME_V2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_SHARED_VOLUME_V2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CHANGE_SPACEPORT_V2(pub i32);
impl windows_core::TypeKind for CLUSTER_CHANGE_SPACEPORT_V2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CHANGE_SPACEPORT_V2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CHANGE_SPACEPORT_V2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CLOUD_TYPE(pub i32);
impl windows_core::TypeKind for CLUSTER_CLOUD_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CLOUD_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CLOUD_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CONTROL_OBJECT(pub i32);
impl windows_core::TypeKind for CLUSTER_CONTROL_OBJECT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CONTROL_OBJECT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CONTROL_OBJECT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_CSV_VOLUME_FAULT_STATE(pub i32);
impl windows_core::TypeKind for CLUSTER_CSV_VOLUME_FAULT_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_CSV_VOLUME_FAULT_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_CSV_VOLUME_FAULT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_ENUM(pub i32);
impl windows_core::TypeKind for CLUSTER_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_GROUP_AUTOFAILBACK_TYPE(pub i32);
impl windows_core::TypeKind for CLUSTER_GROUP_AUTOFAILBACK_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_GROUP_AUTOFAILBACK_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_GROUP_AUTOFAILBACK_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_GROUP_ENUM(pub i32);
impl windows_core::TypeKind for CLUSTER_GROUP_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_GROUP_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_GROUP_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_GROUP_PRIORITY(pub i32);
impl windows_core::TypeKind for CLUSTER_GROUP_PRIORITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_GROUP_PRIORITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_GROUP_PRIORITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_GROUP_STATE(pub i32);
impl windows_core::TypeKind for CLUSTER_GROUP_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_GROUP_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_GROUP_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_MGMT_POINT_RESTYPE(pub i32);
impl windows_core::TypeKind for CLUSTER_MGMT_POINT_RESTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_MGMT_POINT_RESTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_MGMT_POINT_RESTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_MGMT_POINT_TYPE(pub i32);
impl windows_core::TypeKind for CLUSTER_MGMT_POINT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_MGMT_POINT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_MGMT_POINT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_NETINTERFACE_STATE(pub i32);
impl windows_core::TypeKind for CLUSTER_NETINTERFACE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_NETINTERFACE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_NETINTERFACE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_NETWORK_ENUM(pub i32);
impl windows_core::TypeKind for CLUSTER_NETWORK_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_NETWORK_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_NETWORK_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_NETWORK_ROLE(pub i32);
impl windows_core::TypeKind for CLUSTER_NETWORK_ROLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_NETWORK_ROLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_NETWORK_ROLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_NETWORK_STATE(pub i32);
impl windows_core::TypeKind for CLUSTER_NETWORK_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_NETWORK_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_NETWORK_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_NODE_DRAIN_STATUS(pub i32);
impl windows_core::TypeKind for CLUSTER_NODE_DRAIN_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_NODE_DRAIN_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_NODE_DRAIN_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_NODE_ENUM(pub i32);
impl windows_core::TypeKind for CLUSTER_NODE_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_NODE_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_NODE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_NODE_RESUME_FAILBACK_TYPE(pub i32);
impl windows_core::TypeKind for CLUSTER_NODE_RESUME_FAILBACK_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_NODE_RESUME_FAILBACK_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_NODE_RESUME_FAILBACK_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_NODE_STATE(pub i32);
impl windows_core::TypeKind for CLUSTER_NODE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_NODE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_NODE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_NODE_STATUS(pub i32);
impl windows_core::TypeKind for CLUSTER_NODE_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_NODE_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_NODE_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_NOTIFICATIONS_VERSION(pub i32);
impl windows_core::TypeKind for CLUSTER_NOTIFICATIONS_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_NOTIFICATIONS_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_NOTIFICATIONS_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for CLUSTER_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_OBJECT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_PROPERTY_FORMAT(pub i32);
impl windows_core::TypeKind for CLUSTER_PROPERTY_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_PROPERTY_FORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_PROPERTY_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_PROPERTY_SYNTAX(pub u32);
impl windows_core::TypeKind for CLUSTER_PROPERTY_SYNTAX {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_PROPERTY_SYNTAX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_PROPERTY_SYNTAX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_PROPERTY_TYPE(pub i32);
impl windows_core::TypeKind for CLUSTER_PROPERTY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_PROPERTY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_PROPERTY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_QUORUM_TYPE(pub i32);
impl windows_core::TypeKind for CLUSTER_QUORUM_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_QUORUM_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_QUORUM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_QUORUM_VALUE(pub i32);
impl windows_core::TypeKind for CLUSTER_QUORUM_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_QUORUM_VALUE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_QUORUM_VALUE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_REG_COMMAND(pub i32);
impl windows_core::TypeKind for CLUSTER_REG_COMMAND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_REG_COMMAND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_REG_COMMAND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_RESOURCE_APPLICATION_STATE(pub i32);
impl windows_core::TypeKind for CLUSTER_RESOURCE_APPLICATION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_RESOURCE_APPLICATION_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_APPLICATION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_RESOURCE_CLASS(pub i32);
impl windows_core::TypeKind for CLUSTER_RESOURCE_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_RESOURCE_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_RESOURCE_CREATE_FLAGS(pub i32);
impl windows_core::TypeKind for CLUSTER_RESOURCE_CREATE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_RESOURCE_CREATE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_CREATE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION(pub i32);
impl windows_core::TypeKind for CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_RESOURCE_ENUM(pub i32);
impl windows_core::TypeKind for CLUSTER_RESOURCE_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_RESOURCE_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_RESOURCE_RESTART_ACTION(pub i32);
impl windows_core::TypeKind for CLUSTER_RESOURCE_RESTART_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_RESOURCE_RESTART_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_RESTART_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_RESOURCE_STATE(pub i32);
impl windows_core::TypeKind for CLUSTER_RESOURCE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_RESOURCE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_RESOURCE_STATE_CHANGE_REASON(pub i32);
impl windows_core::TypeKind for CLUSTER_RESOURCE_STATE_CHANGE_REASON {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_RESOURCE_STATE_CHANGE_REASON {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_STATE_CHANGE_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_RESOURCE_TYPE_ENUM(pub i32);
impl windows_core::TypeKind for CLUSTER_RESOURCE_TYPE_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_RESOURCE_TYPE_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_RESOURCE_TYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_ROLE(pub i32);
impl windows_core::TypeKind for CLUSTER_ROLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_ROLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_ROLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_ROLE_STATE(pub i32);
impl windows_core::TypeKind for CLUSTER_ROLE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_ROLE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_ROLE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_SETUP_PHASE(pub i32);
impl windows_core::TypeKind for CLUSTER_SETUP_PHASE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_SETUP_PHASE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_SETUP_PHASE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_SETUP_PHASE_SEVERITY(pub i32);
impl windows_core::TypeKind for CLUSTER_SETUP_PHASE_SEVERITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_SETUP_PHASE_SEVERITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_SETUP_PHASE_SEVERITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_SETUP_PHASE_TYPE(pub i32);
impl windows_core::TypeKind for CLUSTER_SETUP_PHASE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_SETUP_PHASE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_SETUP_PHASE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_SHARED_VOLUME_BACKUP_STATE(pub i32);
impl windows_core::TypeKind for CLUSTER_SHARED_VOLUME_BACKUP_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_SHARED_VOLUME_BACKUP_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_SHARED_VOLUME_BACKUP_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(pub i32);
impl windows_core::TypeKind for CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE(pub i32);
impl windows_core::TypeKind for CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_SHARED_VOLUME_STATE(pub i32);
impl windows_core::TypeKind for CLUSTER_SHARED_VOLUME_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_SHARED_VOLUME_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_SHARED_VOLUME_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_STORAGENODE_STATE(pub i32);
impl windows_core::TypeKind for CLUSTER_STORAGENODE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_STORAGENODE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_STORAGENODE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUSTER_UPGRADE_PHASE(pub i32);
impl windows_core::TypeKind for CLUSTER_UPGRADE_PHASE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUSTER_UPGRADE_PHASE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUSTER_UPGRADE_PHASE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUS_AFFINITY_RULE_TYPE(pub i32);
impl windows_core::TypeKind for CLUS_AFFINITY_RULE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUS_AFFINITY_RULE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUS_AFFINITY_RULE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUS_CHARACTERISTICS(pub i32);
impl windows_core::TypeKind for CLUS_CHARACTERISTICS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUS_CHARACTERISTICS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUS_CHARACTERISTICS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUS_FLAGS(pub i32);
impl windows_core::TypeKind for CLUS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUS_GROUP_START_SETTING(pub i32);
impl windows_core::TypeKind for CLUS_GROUP_START_SETTING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUS_GROUP_START_SETTING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUS_GROUP_START_SETTING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUS_RESSUBCLASS(pub i32);
impl windows_core::TypeKind for CLUS_RESSUBCLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUS_RESSUBCLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUS_RESSUBCLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUS_RESSUBCLASS_NETWORK(pub i32);
impl windows_core::TypeKind for CLUS_RESSUBCLASS_NETWORK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUS_RESSUBCLASS_NETWORK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUS_RESSUBCLASS_NETWORK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLUS_RESSUBCLASS_STORAGE(pub i32);
impl windows_core::TypeKind for CLUS_RESSUBCLASS_STORAGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLUS_RESSUBCLASS_STORAGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLUS_RESSUBCLASS_STORAGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FAILURE_TYPE(pub i32);
impl windows_core::TypeKind for FAILURE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FAILURE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FAILURE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILESHARE_CHANGE_ENUM(pub i32);
impl windows_core::TypeKind for FILESHARE_CHANGE_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILESHARE_CHANGE_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILESHARE_CHANGE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GRP_PLACEMENT_OPTIONS(pub i32);
impl windows_core::TypeKind for GRP_PLACEMENT_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GRP_PLACEMENT_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GRP_PLACEMENT_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LOG_LEVEL(pub i32);
impl windows_core::TypeKind for LOG_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LOG_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LOG_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MAINTENANCE_MODE_TYPE_ENUM(pub i32);
impl windows_core::TypeKind for MAINTENANCE_MODE_TYPE_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MAINTENANCE_MODE_TYPE_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MAINTENANCE_MODE_TYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NODE_CLUSTER_STATE(pub i32);
impl windows_core::TypeKind for NODE_CLUSTER_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NODE_CLUSTER_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NODE_CLUSTER_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PLACEMENT_OPTIONS(pub i32);
impl windows_core::TypeKind for PLACEMENT_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PLACEMENT_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PLACEMENT_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RESDLL_CONTEXT_OPERATION_TYPE(pub i32);
impl windows_core::TypeKind for RESDLL_CONTEXT_OPERATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RESDLL_CONTEXT_OPERATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RESDLL_CONTEXT_OPERATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RESOURCE_EXIT_STATE(pub i32);
impl windows_core::TypeKind for RESOURCE_EXIT_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RESOURCE_EXIT_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RESOURCE_EXIT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RESOURCE_MONITOR_STATE(pub i32);
impl windows_core::TypeKind for RESOURCE_MONITOR_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RESOURCE_MONITOR_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RESOURCE_MONITOR_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SR_DISK_REPLICATION_ELIGIBLE(pub i32);
impl windows_core::TypeKind for SR_DISK_REPLICATION_ELIGIBLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SR_DISK_REPLICATION_ELIGIBLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SR_DISK_REPLICATION_ELIGIBLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SR_REPLICATED_DISK_TYPE(pub i32);
impl windows_core::TypeKind for SR_REPLICATED_DISK_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SR_REPLICATED_DISK_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SR_REPLICATED_DISK_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VM_RESDLL_CONTEXT(pub i32);
impl windows_core::TypeKind for VM_RESDLL_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VM_RESDLL_CONTEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VM_RESDLL_CONTEXT").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CLRES_CALLBACK_FUNCTION_TABLE {
    pub LogEvent: PLOG_EVENT_ROUTINE,
    pub SetResourceStatusEx: PSET_RESOURCE_STATUS_ROUTINE_EX,
    pub SetResourceLockedMode: PSET_RESOURCE_LOCKED_MODE_ROUTINE,
    pub SignalFailure: PSIGNAL_FAILURE_ROUTINE,
    pub SetResourceInMemoryNodeLocalProperties: PSET_RESOURCE_INMEMORY_NODELOCAL_PROPERTIES_ROUTINE,
    pub EndControlCall: PEND_CONTROL_CALL,
    pub EndTypeControlCall: PEND_TYPE_CONTROL_CALL,
    pub ExtendControlCall: PEXTEND_RES_CONTROL_CALL,
    pub ExtendTypeControlCall: PEXTEND_RES_TYPE_CONTROL_CALL,
    pub RaiseResTypeNotification: PRAISE_RES_TYPE_NOTIFICATION,
    pub ChangeResourceProcessForDumps: PCHANGE_RESOURCE_PROCESS_FOR_DUMPS,
    pub ChangeResTypeProcessForDumps: PCHANGE_RES_TYPE_PROCESS_FOR_DUMPS,
    pub SetInternalState: PSET_INTERNAL_STATE,
    pub SetResourceLockedModeEx: PSET_RESOURCE_LOCKED_MODE_EX_ROUTINE,
    pub RequestDump: PREQUEST_DUMP_ROUTINE,
    pub SetResourceWprPolicy: PSET_RESOURCE_WPR_POLICY_ROUTINE,
    pub ArmWprWatchdogForCurrentResourceCall: PARM_WPR_WATCHDOG_FOR_CURRENT_RESOURCE_CALL_ROUTINE,
}
impl windows_core::TypeKind for CLRES_CALLBACK_FUNCTION_TABLE {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLRES_CALLBACK_FUNCTION_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy)]
pub struct CLRES_FUNCTION_TABLE {
    pub TableSize: u32,
    pub Version: u32,
    pub Anonymous: CLRES_FUNCTION_TABLE_0,
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::TypeKind for CLRES_FUNCTION_TABLE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Registry")]
impl Default for CLRES_FUNCTION_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy)]
pub union CLRES_FUNCTION_TABLE_0 {
    pub V1Functions: CLRES_V1_FUNCTIONS,
    pub V2Functions: CLRES_V2_FUNCTIONS,
    pub V3Functions: CLRES_V3_FUNCTIONS,
    pub V4Functions: CLRES_V4_FUNCTIONS,
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::TypeKind for CLRES_FUNCTION_TABLE_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Registry")]
impl Default for CLRES_FUNCTION_TABLE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug)]
pub struct CLRES_V1_FUNCTIONS {
    pub Open: POPEN_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_ROUTINE,
    pub Offline: POFFLINE_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub ResourceControl: PRESOURCE_CONTROL_ROUTINE,
    pub ResourceTypeControl: PRESOURCE_TYPE_CONTROL_ROUTINE,
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::TypeKind for CLRES_V1_FUNCTIONS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Registry")]
impl Default for CLRES_V1_FUNCTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug)]
pub struct CLRES_V2_FUNCTIONS {
    pub Open: POPEN_V2_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_V2_ROUTINE,
    pub Offline: POFFLINE_V2_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub ResourceControl: PRESOURCE_CONTROL_ROUTINE,
    pub ResourceTypeControl: PRESOURCE_TYPE_CONTROL_ROUTINE,
    pub Cancel: PCANCEL_ROUTINE,
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::TypeKind for CLRES_V2_FUNCTIONS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Registry")]
impl Default for CLRES_V2_FUNCTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug)]
pub struct CLRES_V3_FUNCTIONS {
    pub Open: POPEN_V2_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_V2_ROUTINE,
    pub Offline: POFFLINE_V2_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub BeginResourceControl: PBEGIN_RESCALL_ROUTINE,
    pub BeginResourceTypeControl: PBEGIN_RESTYPECALL_ROUTINE,
    pub Cancel: PCANCEL_ROUTINE,
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::TypeKind for CLRES_V3_FUNCTIONS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Registry")]
impl Default for CLRES_V3_FUNCTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug)]
pub struct CLRES_V4_FUNCTIONS {
    pub Open: POPEN_V2_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_V2_ROUTINE,
    pub Offline: POFFLINE_V2_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub BeginResourceControl: PBEGIN_RESCALL_ROUTINE,
    pub BeginResourceTypeControl: PBEGIN_RESTYPECALL_ROUTINE,
    pub Cancel: PCANCEL_ROUTINE,
    pub BeginResourceControlAsUser: PBEGIN_RESCALL_AS_USER_ROUTINE,
    pub BeginResourceTypeControlAsUser: PBEGIN_RESTYPECALL_AS_USER_ROUTINE,
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::TypeKind for CLRES_V4_FUNCTIONS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Registry")]
impl Default for CLRES_V4_FUNCTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CLUSAPI_REASON_HANDLER {
    pub lpParameter: *mut core::ffi::c_void,
    pub pfnHandler: PCLUSAPI_PFN_REASON_HANDLER,
}
impl windows_core::TypeKind for CLUSAPI_REASON_HANDLER {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSAPI_REASON_HANDLER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    pub GetTickCount64: u64,
    pub GetSystemTime: super::super::Foundation::SYSTEMTIME,
    pub NodeId: u32,
}
impl windows_core::TypeKind for CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub eReason: CLUSTER_RESOURCE_STATE_CHANGE_REASON,
}
impl windows_core::TypeKind for CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    pub dwFlags: u32,
    pub guidPoolFilter: windows_core::GUID,
}
impl windows_core::TypeKind for CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_BINARY {
    pub Base: CLUSPROP_VALUE,
    pub rgb: [u8; 1],
}
impl windows_core::TypeKind for CLUSPROP_BINARY {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_BINARY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub union CLUSPROP_BUFFER_HELPER {
    pub pb: *mut u8,
    pub pw: *mut u16,
    pub pdw: *mut u32,
    pub pl: *mut i32,
    pub psz: windows_core::PWSTR,
    pub pList: *mut CLUSPROP_LIST,
    pub pSyntax: *mut CLUSPROP_SYNTAX,
    pub pName: *mut CLUSPROP_SZ,
    pub pValue: *mut CLUSPROP_VALUE,
    pub pBinaryValue: *mut CLUSPROP_BINARY,
    pub pWordValue: *mut CLUSPROP_WORD,
    pub pDwordValue: *mut CLUSPROP_DWORD,
    pub pLongValue: *mut CLUSPROP_LONG,
    pub pULargeIntegerValue: *mut CLUSPROP_ULARGE_INTEGER,
    pub pLargeIntegerValue: *mut CLUSPROP_LARGE_INTEGER,
    pub pStringValue: *mut CLUSPROP_SZ,
    pub pMultiSzValue: *mut CLUSPROP_SZ,
    pub pSecurityDescriptor: *mut CLUSPROP_SECURITY_DESCRIPTOR,
    pub pResourceClassValue: *mut CLUSPROP_RESOURCE_CLASS,
    pub pResourceClassInfoValue: *mut CLUSPROP_RESOURCE_CLASS_INFO,
    pub pDiskSignatureValue: *mut CLUSPROP_DWORD,
    pub pScsiAddressValue: *mut CLUSPROP_SCSI_ADDRESS,
    pub pDiskNumberValue: *mut CLUSPROP_DWORD,
    pub pPartitionInfoValue: *mut CLUSPROP_PARTITION_INFO,
    pub pRequiredDependencyValue: *mut CLUSPROP_REQUIRED_DEPENDENCY,
    pub pPartitionInfoValueEx: *mut CLUSPROP_PARTITION_INFO_EX,
    pub pPartitionInfoValueEx2: *mut CLUSPROP_PARTITION_INFO_EX2,
    pub pFileTimeValue: *mut CLUSPROP_FILETIME,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for CLUSPROP_BUFFER_HELPER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for CLUSPROP_BUFFER_HELPER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_DWORD {
    pub Base: CLUSPROP_VALUE,
    pub dw: u32,
}
impl windows_core::TypeKind for CLUSPROP_DWORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_DWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_FILETIME {
    pub Base: CLUSPROP_VALUE,
    pub ft: super::super::Foundation::FILETIME,
}
impl windows_core::TypeKind for CLUSPROP_FILETIME {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_FILETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_FTSET_INFO {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_FTSET_INFO,
}
impl windows_core::TypeKind for CLUSPROP_FTSET_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_FTSET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_LARGE_INTEGER {
    pub Base: CLUSPROP_VALUE,
    pub li: i64,
}
impl windows_core::TypeKind for CLUSPROP_LARGE_INTEGER {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_LARGE_INTEGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_LIST {
    pub nPropertyCount: u32,
    pub PropertyName: CLUSPROP_SZ,
}
impl windows_core::TypeKind for CLUSPROP_LIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_LONG {
    pub Base: CLUSPROP_VALUE,
    pub l: i32,
}
impl windows_core::TypeKind for CLUSPROP_LONG {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_LONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_PARTITION_INFO {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_PARTITION_INFO,
}
impl windows_core::TypeKind for CLUSPROP_PARTITION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_PARTITION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_PARTITION_INFO_EX {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_PARTITION_INFO_EX,
}
impl windows_core::TypeKind for CLUSPROP_PARTITION_INFO_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_PARTITION_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_PARTITION_INFO_EX2 {
    pub Base: CLUSPROP_PARTITION_INFO_EX,
    pub Base2: CLUS_PARTITION_INFO_EX2,
}
impl windows_core::TypeKind for CLUSPROP_PARTITION_INFO_EX2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_PARTITION_INFO_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUSPROP_REQUIRED_DEPENDENCY {
    pub Value: CLUSPROP_VALUE,
    pub ResClass: CLUSPROP_RESOURCE_CLASS,
    pub ResTypeName: CLUSPROP_SZ,
}
impl windows_core::TypeKind for CLUSPROP_REQUIRED_DEPENDENCY {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_REQUIRED_DEPENDENCY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_RESOURCE_CLASS {
    pub Base: CLUSPROP_VALUE,
    pub rc: CLUSTER_RESOURCE_CLASS,
}
impl windows_core::TypeKind for CLUSPROP_RESOURCE_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_RESOURCE_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_RESOURCE_CLASS_INFO {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_RESOURCE_CLASS_INFO,
}
impl windows_core::TypeKind for CLUSPROP_RESOURCE_CLASS_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_RESOURCE_CLASS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_SCSI_ADDRESS {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_SCSI_ADDRESS,
}
impl windows_core::TypeKind for CLUSPROP_SCSI_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_SCSI_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct CLUSPROP_SECURITY_DESCRIPTOR {
    pub Base: CLUSPROP_VALUE,
    pub Anonymous: CLUSPROP_SECURITY_DESCRIPTOR_0,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for CLUSPROP_SECURITY_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for CLUSPROP_SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub union CLUSPROP_SECURITY_DESCRIPTOR_0 {
    pub sd: super::super::Security::SECURITY_DESCRIPTOR_RELATIVE,
    pub rgbSecurityDescriptor: [u8; 1],
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for CLUSPROP_SECURITY_DESCRIPTOR_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for CLUSPROP_SECURITY_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUSPROP_SYNTAX {
    pub dw: u32,
    pub Anonymous: CLUSPROP_SYNTAX_0,
}
impl windows_core::TypeKind for CLUSPROP_SYNTAX {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_SYNTAX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSPROP_SYNTAX_0 {
    pub wFormat: u16,
    pub wType: u16,
}
impl windows_core::TypeKind for CLUSPROP_SYNTAX_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_SYNTAX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_SZ {
    pub Base: CLUSPROP_VALUE,
    pub sz: [u16; 1],
}
impl windows_core::TypeKind for CLUSPROP_SZ {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_SZ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_ULARGE_INTEGER {
    pub Base: CLUSPROP_VALUE,
    pub li: u64,
}
impl windows_core::TypeKind for CLUSPROP_ULARGE_INTEGER {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_ULARGE_INTEGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_VALUE {
    pub Syntax: CLUSPROP_SYNTAX,
    pub cbLength: u32,
}
impl windows_core::TypeKind for CLUSPROP_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_WORD {
    pub Base: CLUSPROP_VALUE,
    pub w: u16,
}
impl windows_core::TypeKind for CLUSPROP_WORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSPROP_WORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTERVERSIONINFO {
    pub dwVersionInfoSize: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub szVendorId: [u16; 64],
    pub szCSDVersion: [u16; 64],
    pub dwClusterHighestVersion: u32,
    pub dwClusterLowestVersion: u32,
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl windows_core::TypeKind for CLUSTERVERSIONINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTERVERSIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTERVERSIONINFO_NT4 {
    pub dwVersionInfoSize: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub szVendorId: [u16; 64],
    pub szCSDVersion: [u16; 64],
}
impl windows_core::TypeKind for CLUSTERVERSIONINFO_NT4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTERVERSIONINFO_NT4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_AVAILABILITY_SET_CONFIG {
    pub dwVersion: u32,
    pub dwUpdateDomains: u32,
    pub dwFaultDomains: u32,
    pub bReserveSpareNode: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for CLUSTER_AVAILABILITY_SET_CONFIG {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_AVAILABILITY_SET_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_BATCH_COMMAND {
    pub Command: CLUSTER_REG_COMMAND,
    pub dwOptions: u32,
    pub wzName: windows_core::PCWSTR,
    pub lpData: *const u8,
    pub cbData: u32,
}
impl windows_core::TypeKind for CLUSTER_BATCH_COMMAND {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_BATCH_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_CREATE_GROUP_INFO {
    pub dwVersion: u32,
    pub groupType: CLUSGROUP_TYPE,
}
impl windows_core::TypeKind for CLUSTER_CREATE_GROUP_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_CREATE_GROUP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_ENUM_ITEM {
    pub dwVersion: u32,
    pub dwType: u32,
    pub cbId: u32,
    pub lpszId: windows_core::PWSTR,
    pub cbName: u32,
    pub lpszName: windows_core::PWSTR,
}
impl windows_core::TypeKind for CLUSTER_ENUM_ITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_ENUM_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_GROUP_ENUM_ITEM {
    pub dwVersion: u32,
    pub cbId: u32,
    pub lpszId: windows_core::PWSTR,
    pub cbName: u32,
    pub lpszName: windows_core::PWSTR,
    pub state: CLUSTER_GROUP_STATE,
    pub cbOwnerNode: u32,
    pub lpszOwnerNode: windows_core::PWSTR,
    pub dwFlags: u32,
    pub cbProperties: u32,
    pub pProperties: *mut core::ffi::c_void,
    pub cbRoProperties: u32,
    pub pRoProperties: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for CLUSTER_GROUP_ENUM_ITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_GROUP_ENUM_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_HEALTH_FAULT {
    pub Id: windows_core::PWSTR,
    pub ErrorType: u32,
    pub ErrorCode: u32,
    pub Description: windows_core::PWSTR,
    pub Provider: windows_core::PWSTR,
    pub Flags: u32,
    pub Reserved: u32,
}
impl windows_core::TypeKind for CLUSTER_HEALTH_FAULT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_HEALTH_FAULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_HEALTH_FAULT_ARRAY {
    pub numFaults: u32,
    pub faults: *mut CLUSTER_HEALTH_FAULT,
}
impl windows_core::TypeKind for CLUSTER_HEALTH_FAULT_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_HEALTH_FAULT_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_IP_ENTRY {
    pub lpszIpAddress: windows_core::PCWSTR,
    pub dwPrefixLength: u32,
}
impl windows_core::TypeKind for CLUSTER_IP_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_IP_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_MEMBERSHIP_INFO {
    pub HasQuorum: super::super::Foundation::BOOL,
    pub UpnodesSize: u32,
    pub Upnodes: [u8; 1],
}
impl windows_core::TypeKind for CLUSTER_MEMBERSHIP_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_MEMBERSHIP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_READ_BATCH_COMMAND {
    pub Command: CLUSTER_REG_COMMAND,
    pub dwOptions: u32,
    pub wzSubkeyName: windows_core::PCWSTR,
    pub wzValueName: windows_core::PCWSTR,
    pub lpData: *const u8,
    pub cbData: u32,
}
impl windows_core::TypeKind for CLUSTER_READ_BATCH_COMMAND {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_READ_BATCH_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_RESOURCE_ENUM_ITEM {
    pub dwVersion: u32,
    pub cbId: u32,
    pub lpszId: windows_core::PWSTR,
    pub cbName: u32,
    pub lpszName: windows_core::PWSTR,
    pub cbOwnerGroupName: u32,
    pub lpszOwnerGroupName: windows_core::PWSTR,
    pub cbOwnerGroupId: u32,
    pub lpszOwnerGroupId: windows_core::PWSTR,
    pub cbProperties: u32,
    pub pProperties: *mut core::ffi::c_void,
    pub cbRoProperties: u32,
    pub pRoProperties: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for CLUSTER_RESOURCE_ENUM_ITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_RESOURCE_ENUM_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_SET_PASSWORD_STATUS {
    pub NodeId: u32,
    pub SetAttempted: super::super::Foundation::BOOLEAN,
    pub ReturnStatus: u32,
}
impl windows_core::TypeKind for CLUSTER_SET_PASSWORD_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_SET_PASSWORD_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    pub Base: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME,
    pub Base2: CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME,
}
impl windows_core::TypeKind for CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT {
    pub Base: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME,
    pub Base2: CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME,
}
impl windows_core::TypeKind for CLUSTER_SHARED_VOLUME_RENAME_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    pub NewVolumeName: [u16; 260],
    pub NewVolumeGuid: [u16; 50],
}
impl windows_core::TypeKind for CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    pub NewVolumeName: [u16; 260],
}
impl windows_core::TypeKind for CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    pub InputType: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE,
    pub Anonymous: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0,
}
impl windows_core::TypeKind for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {
    pub VolumeOffset: u64,
    pub VolumeId: [u16; 260],
    pub VolumeName: [u16; 260],
    pub VolumeGuid: [u16; 50],
}
impl windows_core::TypeKind for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_SHARED_VOLUME_STATE_INFO {
    pub szVolumeName: [u16; 260],
    pub szNodeName: [u16; 260],
    pub VolumeState: CLUSTER_SHARED_VOLUME_STATE,
}
impl windows_core::TypeKind for CLUSTER_SHARED_VOLUME_STATE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_SHARED_VOLUME_STATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    pub szVolumeName: [u16; 260],
    pub szNodeName: [u16; 260],
    pub VolumeState: CLUSTER_SHARED_VOLUME_STATE,
    pub szVolumeFriendlyName: [u16; 260],
    pub RedirectedIOReason: u64,
    pub VolumeRedirectedIOReason: u64,
}
impl windows_core::TypeKind for CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_VALIDATE_CSV_FILENAME {
    pub szFileName: [u16; 1],
}
impl windows_core::TypeKind for CLUSTER_VALIDATE_CSV_FILENAME {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_VALIDATE_CSV_FILENAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_VALIDATE_DIRECTORY {
    pub szPath: [u16; 1],
}
impl windows_core::TypeKind for CLUSTER_VALIDATE_DIRECTORY {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_VALIDATE_DIRECTORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_VALIDATE_NETNAME {
    pub szNetworkName: [u16; 1],
}
impl windows_core::TypeKind for CLUSTER_VALIDATE_NETNAME {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_VALIDATE_NETNAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUSTER_VALIDATE_PATH {
    pub szPath: [u16; 1],
}
impl windows_core::TypeKind for CLUSTER_VALIDATE_PATH {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUSTER_VALIDATE_PATH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_CHKDSK_INFO {
    pub PartitionNumber: u32,
    pub ChkdskState: u32,
    pub FileIdCount: u32,
    pub FileIdList: [u64; 1],
}
impl windows_core::TypeKind for CLUS_CHKDSK_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_CHKDSK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    pub FileServerName: [u16; 16],
}
impl windows_core::TypeKind for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    pub FileServerName: [u16; 260],
}
impl windows_core::TypeKind for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_CSV_MAINTENANCE_MODE_INFO {
    pub InMaintenance: super::super::Foundation::BOOL,
    pub VolumeName: [u16; 260],
}
impl windows_core::TypeKind for CLUS_CSV_MAINTENANCE_MODE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_CSV_MAINTENANCE_MODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_CSV_VOLUME_INFO {
    pub VolumeOffset: u64,
    pub PartitionNumber: u32,
    pub FaultState: CLUSTER_CSV_VOLUME_FAULT_STATE,
    pub BackupState: CLUSTER_SHARED_VOLUME_BACKUP_STATE,
    pub szVolumeFriendlyName: [u16; 260],
    pub szVolumeName: [u16; 50],
}
impl windows_core::TypeKind for CLUS_CSV_VOLUME_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_CSV_VOLUME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_CSV_VOLUME_NAME {
    pub VolumeOffset: i64,
    pub szVolumeName: [u16; 260],
    pub szRootPath: [u16; 263],
}
impl windows_core::TypeKind for CLUS_CSV_VOLUME_NAME {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_CSV_VOLUME_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_DISK_NUMBER_INFO {
    pub DiskNumber: u32,
    pub BytesPerSector: u32,
}
impl windows_core::TypeKind for CLUS_DISK_NUMBER_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_DISK_NUMBER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_DNN_LEADER_STATUS {
    pub IsOnline: super::super::Foundation::BOOL,
    pub IsFileServerPresent: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for CLUS_DNN_LEADER_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_DNN_LEADER_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_DNN_SODAFS_CLONE_STATUS {
    pub NodeId: u32,
    pub Status: CLUSTER_RESOURCE_STATE,
}
impl windows_core::TypeKind for CLUS_DNN_SODAFS_CLONE_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_DNN_SODAFS_CLONE_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_FORCE_QUORUM_INFO {
    pub dwSize: u32,
    pub dwNodeBitMask: u32,
    pub dwMaxNumberofNodes: u32,
    pub multiszNodeList: [u16; 1],
}
impl windows_core::TypeKind for CLUS_FORCE_QUORUM_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_FORCE_QUORUM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_FTSET_INFO {
    pub dwRootSignature: u32,
    pub dwFtType: u32,
}
impl windows_core::TypeKind for CLUS_FTSET_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_FTSET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_MAINTENANCE_MODE_INFO {
    pub InMaintenance: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for CLUS_MAINTENANCE_MODE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_MAINTENANCE_MODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_MAINTENANCE_MODE_INFOEX {
    pub InMaintenance: super::super::Foundation::BOOL,
    pub MaintainenceModeType: MAINTENANCE_MODE_TYPE_ENUM,
    pub InternalState: CLUSTER_RESOURCE_STATE,
    pub Signature: u32,
}
impl windows_core::TypeKind for CLUS_MAINTENANCE_MODE_INFOEX {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_MAINTENANCE_MODE_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_NETNAME_IP_INFO_ENTRY {
    pub NodeId: u32,
    pub AddressSize: u32,
    pub Address: [u8; 1],
}
impl windows_core::TypeKind for CLUS_NETNAME_IP_INFO_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_NETNAME_IP_INFO_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    pub szName: [u16; 64],
    pub NumEntries: u32,
    pub IpInfo: [CLUS_NETNAME_IP_INFO_ENTRY; 1],
}
impl windows_core::TypeKind for CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_NETNAME_PWD_INFO {
    pub Flags: u32,
    pub Password: [u16; 16],
    pub CreatingDC: [u16; 258],
    pub ObjectGuid: [u16; 64],
}
impl windows_core::TypeKind for CLUS_NETNAME_PWD_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_NETNAME_PWD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_NETNAME_PWD_INFOEX {
    pub Flags: u32,
    pub Password: [u16; 128],
    pub CreatingDC: [u16; 258],
    pub ObjectGuid: [u16; 64],
}
impl windows_core::TypeKind for CLUS_NETNAME_PWD_INFOEX {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_NETNAME_PWD_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_NETNAME_VS_TOKEN_INFO {
    pub ProcessID: u32,
    pub DesiredAccess: u32,
    pub InheritHandle: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for CLUS_NETNAME_VS_TOKEN_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_NETNAME_VS_TOKEN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_PARTITION_INFO {
    pub dwFlags: u32,
    pub szDeviceName: [u16; 260],
    pub szVolumeLabel: [u16; 260],
    pub dwSerialNumber: u32,
    pub rgdwMaximumComponentLength: u32,
    pub dwFileSystemFlags: u32,
    pub szFileSystem: [u16; 32],
}
impl windows_core::TypeKind for CLUS_PARTITION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_PARTITION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_PARTITION_INFO_EX {
    pub dwFlags: u32,
    pub szDeviceName: [u16; 260],
    pub szVolumeLabel: [u16; 260],
    pub dwSerialNumber: u32,
    pub rgdwMaximumComponentLength: u32,
    pub dwFileSystemFlags: u32,
    pub szFileSystem: [u16; 32],
    pub TotalSizeInBytes: u64,
    pub FreeSizeInBytes: u64,
    pub DeviceNumber: u32,
    pub PartitionNumber: u32,
    pub VolumeGuid: windows_core::GUID,
}
impl windows_core::TypeKind for CLUS_PARTITION_INFO_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_PARTITION_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_PARTITION_INFO_EX2 {
    pub GptPartitionId: windows_core::GUID,
    pub szPartitionName: [u16; 260],
    pub EncryptionFlags: u32,
}
impl windows_core::TypeKind for CLUS_PARTITION_INFO_EX2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_PARTITION_INFO_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_PROVIDER_STATE_CHANGE_INFO {
    pub dwSize: u32,
    pub resourceState: CLUSTER_RESOURCE_STATE,
    pub szProviderId: [u16; 1],
}
impl windows_core::TypeKind for CLUS_PROVIDER_STATE_CHANGE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_PROVIDER_STATE_CHANGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUS_RESOURCE_CLASS_INFO {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0,
}
impl windows_core::TypeKind for CLUS_RESOURCE_CLASS_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_RESOURCE_CLASS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUS_RESOURCE_CLASS_INFO_0 {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0_0,
    pub li: u64,
}
impl windows_core::TypeKind for CLUS_RESOURCE_CLASS_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_RESOURCE_CLASS_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUS_RESOURCE_CLASS_INFO_0_0 {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0_0_0,
    pub SubClass: u32,
}
impl windows_core::TypeKind for CLUS_RESOURCE_CLASS_INFO_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_RESOURCE_CLASS_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUS_RESOURCE_CLASS_INFO_0_0_0 {
    pub dw: u32,
    pub rc: CLUSTER_RESOURCE_CLASS,
}
impl windows_core::TypeKind for CLUS_RESOURCE_CLASS_INFO_0_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_RESOURCE_CLASS_INFO_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUS_SCSI_ADDRESS {
    pub Anonymous: CLUS_SCSI_ADDRESS_0,
}
impl windows_core::TypeKind for CLUS_SCSI_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_SCSI_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUS_SCSI_ADDRESS_0 {
    pub Anonymous: CLUS_SCSI_ADDRESS_0_0,
    pub dw: u32,
}
impl windows_core::TypeKind for CLUS_SCSI_ADDRESS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_SCSI_ADDRESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_SCSI_ADDRESS_0_0 {
    pub PortNumber: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
}
impl windows_core::TypeKind for CLUS_SCSI_ADDRESS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_SCSI_ADDRESS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_SET_MAINTENANCE_MODE_INPUT {
    pub InMaintenance: super::super::Foundation::BOOL,
    pub ExtraParameterSize: u32,
    pub ExtraParameter: [u8; 1],
}
impl windows_core::TypeKind for CLUS_SET_MAINTENANCE_MODE_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_SET_MAINTENANCE_MODE_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_SHARED_VOLUME_BACKUP_MODE {
    pub BackupState: CLUSTER_SHARED_VOLUME_BACKUP_STATE,
    pub DelayTimerInSecs: u32,
    pub VolumeName: [u16; 260],
}
impl windows_core::TypeKind for CLUS_SHARED_VOLUME_BACKUP_MODE {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_SHARED_VOLUME_BACKUP_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_STARTING_PARAMS {
    pub dwSize: u32,
    pub bForm: super::super::Foundation::BOOL,
    pub bFirst: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for CLUS_STARTING_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_STARTING_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    pub AvailDrivelettersMask: u32,
}
impl windows_core::TypeKind for CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_STORAGE_REMAP_DRIVELETTER {
    pub CurrentDriveLetterMask: u32,
    pub TargetDriveLetterMask: u32,
}
impl windows_core::TypeKind for CLUS_STORAGE_REMAP_DRIVELETTER {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_STORAGE_REMAP_DRIVELETTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_STORAGE_SET_DRIVELETTER {
    pub PartitionNumber: u32,
    pub DriveLetterMask: u32,
}
impl windows_core::TypeKind for CLUS_STORAGE_SET_DRIVELETTER {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_STORAGE_SET_DRIVELETTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLUS_WORKER {
    pub hThread: super::super::Foundation::HANDLE,
    pub Terminate: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for CLUS_WORKER {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLUS_WORKER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CREATE_CLUSTER_CONFIG {
    pub dwVersion: u32,
    pub lpszClusterName: windows_core::PCWSTR,
    pub cNodes: u32,
    pub ppszNodeNames: *const windows_core::PCWSTR,
    pub cIpEntries: u32,
    pub pIpEntries: *mut CLUSTER_IP_ENTRY,
    pub fEmptyCluster: super::super::Foundation::BOOLEAN,
    pub managementPointType: CLUSTER_MGMT_POINT_TYPE,
    pub managementPointResType: CLUSTER_MGMT_POINT_RESTYPE,
}
impl windows_core::TypeKind for CREATE_CLUSTER_CONFIG {
    type TypeKind = windows_core::CopyType;
}
impl Default for CREATE_CLUSTER_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CREATE_CLUSTER_NAME_ACCOUNT {
    pub dwVersion: u32,
    pub lpszClusterName: windows_core::PCWSTR,
    pub dwFlags: u32,
    pub pszUserName: windows_core::PCWSTR,
    pub pszPassword: windows_core::PCWSTR,
    pub pszDomain: windows_core::PCWSTR,
    pub managementPointType: CLUSTER_MGMT_POINT_TYPE,
    pub managementPointResType: CLUSTER_MGMT_POINT_RESTYPE,
    pub bUpgradeVCOs: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for CREATE_CLUSTER_NAME_ACCOUNT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CREATE_CLUSTER_NAME_ACCOUNT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ClusApplication: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606e5_2631_11d1_89f1_00a0c90d061e);
pub const ClusCryptoKeys: windows_core::GUID = windows_core::GUID::from_u128(0xf2e6072b_2631_11d1_89f1_00a0c90d061e);
pub const ClusDisk: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60723_2631_11d1_89f1_00a0c90d061e);
pub const ClusDisks: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60725_2631_11d1_89f1_00a0c90d061e);
pub const ClusNetInterface: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606ed_2631_11d1_89f1_00a0c90d061e);
pub const ClusNetInterfaces: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606ef_2631_11d1_89f1_00a0c90d061e);
pub const ClusNetwork: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606f1_2631_11d1_89f1_00a0c90d061e);
pub const ClusNetworkNetInterfaces: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606f5_2631_11d1_89f1_00a0c90d061e);
pub const ClusNetworks: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606f3_2631_11d1_89f1_00a0c90d061e);
pub const ClusNode: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606f7_2631_11d1_89f1_00a0c90d061e);
pub const ClusNodeNetInterfaces: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606fb_2631_11d1_89f1_00a0c90d061e);
pub const ClusNodes: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606f9_2631_11d1_89f1_00a0c90d061e);
pub const ClusPartition: windows_core::GUID = windows_core::GUID::from_u128(0xf2e6071f_2631_11d1_89f1_00a0c90d061e);
pub const ClusPartitionEx: windows_core::GUID = windows_core::GUID::from_u128(0x53d51d26_b51b_4a79_b2c3_5048d93a98fc);
pub const ClusPartitions: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60721_2631_11d1_89f1_00a0c90d061e);
pub const ClusProperties: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606ff_2631_11d1_89f1_00a0c90d061e);
pub const ClusProperty: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606fd_2631_11d1_89f1_00a0c90d061e);
pub const ClusPropertyValue: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60719_2631_11d1_89f1_00a0c90d061e);
pub const ClusPropertyValueData: windows_core::GUID = windows_core::GUID::from_u128(0xf2e6071d_2631_11d1_89f1_00a0c90d061e);
pub const ClusPropertyValues: windows_core::GUID = windows_core::GUID::from_u128(0xf2e6071b_2631_11d1_89f1_00a0c90d061e);
pub const ClusRefObject: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60701_2631_11d1_89f1_00a0c90d061e);
pub const ClusRegistryKeys: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60729_2631_11d1_89f1_00a0c90d061e);
pub const ClusResDependencies: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60703_2631_11d1_89f1_00a0c90d061e);
pub const ClusResDependents: windows_core::GUID = windows_core::GUID::from_u128(0xf2e6072d_2631_11d1_89f1_00a0c90d061e);
pub const ClusResGroup: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60705_2631_11d1_89f1_00a0c90d061e);
pub const ClusResGroupPreferredOwnerNodes: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606e7_2631_11d1_89f1_00a0c90d061e);
pub const ClusResGroupResources: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606e9_2631_11d1_89f1_00a0c90d061e);
pub const ClusResGroups: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60707_2631_11d1_89f1_00a0c90d061e);
pub const ClusResPossibleOwnerNodes: windows_core::GUID = windows_core::GUID::from_u128(0xf2e6070d_2631_11d1_89f1_00a0c90d061e);
pub const ClusResType: windows_core::GUID = windows_core::GUID::from_u128(0xf2e6070f_2631_11d1_89f1_00a0c90d061e);
pub const ClusResTypePossibleOwnerNodes: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60717_2631_11d1_89f1_00a0c90d061e);
pub const ClusResTypeResources: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60713_2631_11d1_89f1_00a0c90d061e);
pub const ClusResTypes: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60711_2631_11d1_89f1_00a0c90d061e);
pub const ClusResource: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60709_2631_11d1_89f1_00a0c90d061e);
pub const ClusResources: windows_core::GUID = windows_core::GUID::from_u128(0xf2e6070b_2631_11d1_89f1_00a0c90d061e);
pub const ClusScsiAddress: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60727_2631_11d1_89f1_00a0c90d061e);
pub const ClusVersion: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60715_2631_11d1_89f1_00a0c90d061e);
pub const Cluster: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606e3_2631_11d1_89f1_00a0c90d061e);
pub const ClusterNames: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606eb_2631_11d1_89f1_00a0c90d061e);
pub const DomainNames: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606e1_2631_11d1_89f1_00a0c90d061e);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILESHARE_CHANGE {
    pub Change: FILESHARE_CHANGE_ENUM,
    pub ShareName: [u16; 84],
}
impl windows_core::TypeKind for FILESHARE_CHANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILESHARE_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILESHARE_CHANGE_LIST {
    pub NumEntries: u32,
    pub ChangeEntry: [FILESHARE_CHANGE; 1],
}
impl windows_core::TypeKind for FILESHARE_CHANGE_LIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILESHARE_CHANGE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GET_OPERATION_CONTEXT_PARAMS {
    pub Size: u32,
    pub Version: u32,
    pub Type: RESDLL_CONTEXT_OPERATION_TYPE,
    pub Priority: u32,
}
impl windows_core::TypeKind for GET_OPERATION_CONTEXT_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for GET_OPERATION_CONTEXT_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GROUP_FAILURE_INFO {
    pub dwFailoverAttemptsRemaining: u32,
    pub dwFailoverPeriodRemaining: u32,
}
impl windows_core::TypeKind for GROUP_FAILURE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for GROUP_FAILURE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GROUP_FAILURE_INFO_BUFFER {
    pub dwVersion: u32,
    pub Info: GROUP_FAILURE_INFO,
}
impl windows_core::TypeKind for GROUP_FAILURE_INFO_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for GROUP_FAILURE_INFO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCHANGE(pub isize);
impl Default for HCHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HCHANGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCLUSCRYPTPROVIDER(pub isize);
impl Default for HCLUSCRYPTPROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HCLUSCRYPTPROVIDER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCLUSENUM(pub isize);
impl Default for HCLUSENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HCLUSENUM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCLUSENUMEX(pub isize);
impl Default for HCLUSENUMEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HCLUSENUMEX {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCLUSTER(pub isize);
impl Default for HCLUSTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HCLUSTER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HGROUP(pub isize);
impl Default for HGROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HGROUP {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HGROUPENUM(pub isize);
impl Default for HGROUPENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HGROUPENUM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HGROUPENUMEX(pub isize);
impl Default for HGROUPENUMEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HGROUPENUMEX {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HGROUPSET(pub isize);
impl Default for HGROUPSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HGROUPSET {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HGROUPSETENUM(pub isize);
impl Default for HGROUPSETENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HGROUPSETENUM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HNETINTERFACE(pub isize);
impl Default for HNETINTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HNETINTERFACE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HNETINTERFACEENUM(pub isize);
impl Default for HNETINTERFACEENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HNETINTERFACEENUM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HNETWORK(pub isize);
impl Default for HNETWORK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HNETWORK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HNETWORKENUM(pub isize);
impl Default for HNETWORKENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HNETWORKENUM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HNODE(pub isize);
impl Default for HNODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HNODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HNODEENUM(pub isize);
impl Default for HNODEENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HNODEENUM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HNODEENUMEX(pub isize);
impl Default for HNODEENUMEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HNODEENUMEX {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HREGBATCH(pub isize);
impl Default for HREGBATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HREGBATCH {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HREGBATCHNOTIFICATION(pub isize);
impl Default for HREGBATCHNOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HREGBATCHNOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HREGBATCHPORT(pub isize);
impl Default for HREGBATCHPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HREGBATCHPORT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HREGREADBATCH(pub isize);
impl Default for HREGREADBATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HREGREADBATCH {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HREGREADBATCHREPLY(pub isize);
impl Default for HREGREADBATCHREPLY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HREGREADBATCHREPLY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HRESENUM(pub isize);
impl Default for HRESENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HRESENUM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HRESENUMEX(pub isize);
impl Default for HRESENUMEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HRESENUMEX {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HRESOURCE(pub isize);
impl Default for HRESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HRESOURCE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HRESTYPEENUM(pub isize);
impl Default for HRESTYPEENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HRESTYPEENUM {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MONITOR_STATE {
    pub LastUpdate: i64,
    pub State: RESOURCE_MONITOR_STATE,
    pub ActiveResource: super::super::Foundation::HANDLE,
    pub ResmonStop: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for MONITOR_STATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for MONITOR_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NOTIFY_FILTER_AND_TYPE {
    pub dwObjectType: u32,
    pub FilterFlags: i64,
}
impl windows_core::TypeKind for NOTIFY_FILTER_AND_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl Default for NOTIFY_FILTER_AND_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NodeUtilizationInfoElement {
    pub Id: u64,
    pub AvailableMemory: u64,
    pub AvailableMemoryAfterReclamation: u64,
}
impl windows_core::TypeKind for NodeUtilizationInfoElement {
    type TypeKind = windows_core::CopyType;
}
impl Default for NodeUtilizationInfoElement {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct POST_UPGRADE_VERSION_INFO {
    pub newMajorVersion: u32,
    pub newUpgradeVersion: u32,
    pub oldMajorVersion: u32,
    pub oldUpgradeVersion: u32,
    pub reserved: u32,
}
impl windows_core::TypeKind for POST_UPGRADE_VERSION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for POST_UPGRADE_VERSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PaxosTagCStruct {
    pub __padding__PaxosTagVtable: u64,
    pub __padding__NextEpochVtable: u64,
    pub __padding__NextEpoch_DateTimeVtable: u64,
    pub NextEpoch_DateTime_ticks: u64,
    pub NextEpoch_Value: i32,
    pub __padding__BoundryNextEpoch: u32,
    pub __padding__EpochVtable: u64,
    pub __padding__Epoch_DateTimeVtable: u64,
    pub Epoch_DateTime_ticks: u64,
    pub Epoch_Value: i32,
    pub __padding__BoundryEpoch: u32,
    pub Sequence: i32,
    pub __padding__BoundrySequence: u32,
}
impl windows_core::TypeKind for PaxosTagCStruct {
    type TypeKind = windows_core::CopyType;
}
impl Default for PaxosTagCStruct {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RESOURCE_FAILURE_INFO {
    pub dwRestartAttemptsRemaining: u32,
    pub dwRestartPeriodRemaining: u32,
}
impl windows_core::TypeKind for RESOURCE_FAILURE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESOURCE_FAILURE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RESOURCE_FAILURE_INFO_BUFFER {
    pub dwVersion: u32,
    pub Info: RESOURCE_FAILURE_INFO,
}
impl windows_core::TypeKind for RESOURCE_FAILURE_INFO_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESOURCE_FAILURE_INFO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RESOURCE_STATUS {
    pub ResourceState: CLUSTER_RESOURCE_STATE,
    pub CheckPoint: u32,
    pub WaitHint: u32,
    pub EventHandle: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for RESOURCE_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESOURCE_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RESOURCE_STATUS_EX {
    pub ResourceState: CLUSTER_RESOURCE_STATE,
    pub CheckPoint: u32,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub ApplicationSpecificErrorCode: u32,
    pub Flags: u32,
    pub WaitHint: u32,
}
impl windows_core::TypeKind for RESOURCE_STATUS_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESOURCE_STATUS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    pub isTerminalFailure: super::super::Foundation::BOOL,
    pub restartPeriodRemaining: u32,
}
impl windows_core::TypeKind for RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RESUTIL_FILETIME_DATA {
    pub Default: super::super::Foundation::FILETIME,
    pub Minimum: super::super::Foundation::FILETIME,
    pub Maximum: super::super::Foundation::FILETIME,
}
impl windows_core::TypeKind for RESUTIL_FILETIME_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESUTIL_FILETIME_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RESUTIL_LARGEINT_DATA {
    pub Default: i64,
    pub Minimum: i64,
    pub Maximum: i64,
}
impl windows_core::TypeKind for RESUTIL_LARGEINT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESUTIL_LARGEINT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RESUTIL_PROPERTY_ITEM {
    pub Name: windows_core::PWSTR,
    pub KeyName: windows_core::PWSTR,
    pub Format: u32,
    pub Anonymous: RESUTIL_PROPERTY_ITEM_0,
    pub Minimum: u32,
    pub Maximum: u32,
    pub Flags: u32,
    pub Offset: u32,
}
impl windows_core::TypeKind for RESUTIL_PROPERTY_ITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESUTIL_PROPERTY_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RESUTIL_PROPERTY_ITEM_0 {
    pub DefaultPtr: usize,
    pub Default: u32,
    pub lpDefault: *mut core::ffi::c_void,
    pub LargeIntData: *mut RESUTIL_LARGEINT_DATA,
    pub ULargeIntData: *mut RESUTIL_ULARGEINT_DATA,
    pub FileTimeData: *mut RESUTIL_FILETIME_DATA,
}
impl windows_core::TypeKind for RESUTIL_PROPERTY_ITEM_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESUTIL_PROPERTY_ITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RESUTIL_ULARGEINT_DATA {
    pub Default: u64,
    pub Minimum: u64,
    pub Maximum: u64,
}
impl windows_core::TypeKind for RESUTIL_ULARGEINT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESUTIL_ULARGEINT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceUtilizationInfoElement {
    pub PhysicalNumaId: u64,
    pub CurrentMemory: u64,
}
impl windows_core::TypeKind for ResourceUtilizationInfoElement {
    type TypeKind = windows_core::CopyType;
}
impl Default for ResourceUtilizationInfoElement {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    pub ReplicationGroupName: [u16; 260],
    pub Description: [u16; 260],
    pub LogPath: [u16; 260],
    pub MaxLogSizeInBytes: u64,
    pub LogType: u16,
    pub ReplicationMode: u32,
    pub MinimumPartnersInSync: u32,
    pub EnableWriteConsistency: super::super::Foundation::BOOLEAN,
    pub EnableEncryption: super::super::Foundation::BOOLEAN,
    pub EnableCompression: super::super::Foundation::BOOLEAN,
    pub CertificateThumbprint: [u16; 260],
    pub VolumeNameCount: u32,
    pub VolumeNames: [u16; 260],
}
impl windows_core::TypeKind for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    type TypeKind = windows_core::CopyType;
}
impl Default for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    pub Result: u32,
    pub ErrorString: [u16; 260],
}
impl windows_core::TypeKind for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SR_RESOURCE_TYPE_DISK_INFO {
    pub Reason: SR_DISK_REPLICATION_ELIGIBLE,
    pub DiskGuid: windows_core::GUID,
}
impl windows_core::TypeKind for SR_RESOURCE_TYPE_DISK_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SR_RESOURCE_TYPE_DISK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    pub Count: u16,
    pub DiskInfo: [SR_RESOURCE_TYPE_DISK_INFO; 1],
}
impl windows_core::TypeKind for SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    pub DataDiskGuid: windows_core::GUID,
    pub IncludeOfflineDisks: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    pub DataDiskGuid: windows_core::GUID,
    pub IncludeAvailableStoargeDisks: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    pub SourceDataDiskGuid: windows_core::GUID,
    pub TargetReplicationGroupGuid: windows_core::GUID,
    pub SkipConnectivityCheck: super::super::Foundation::BOOLEAN,
    pub IncludeOfflineDisks: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SR_RESOURCE_TYPE_REPLICATED_DISK {
    pub Type: SR_REPLICATED_DISK_TYPE,
    pub ClusterDiskResourceGuid: windows_core::GUID,
    pub ReplicationGroupId: windows_core::GUID,
    pub ReplicationGroupName: [u16; 260],
}
impl windows_core::TypeKind for SR_RESOURCE_TYPE_REPLICATED_DISK {
    type TypeKind = windows_core::CopyType;
}
impl Default for SR_RESOURCE_TYPE_REPLICATED_DISK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    pub Count: u16,
    pub ReplicatedDisks: [SR_RESOURCE_TYPE_REPLICATED_DISK; 1],
}
impl windows_core::TypeKind for SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    pub Count: u32,
    pub PartitionArray: [SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO; 1],
}
impl windows_core::TypeKind for SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    pub PartitionOffset: u64,
    pub Capabilities: u32,
}
impl windows_core::TypeKind for SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WitnessTagHelper {
    pub Version: i32,
    pub paxosToValidate: PaxosTagCStruct,
}
impl windows_core::TypeKind for WitnessTagHelper {
    type TypeKind = windows_core::CopyType;
}
impl Default for WitnessTagHelper {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WitnessTagUpdateHelper {
    pub Version: i32,
    pub paxosToSet: PaxosTagCStruct,
    pub paxosToValidate: PaxosTagCStruct,
}
impl windows_core::TypeKind for WitnessTagUpdateHelper {
    type TypeKind = windows_core::CopyType;
}
impl Default for WitnessTagUpdateHelper {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPGROUP_CALLBACK_EX = Option<unsafe extern "system" fn(param0: HCLUSTER, param1: HGROUP, param2: HGROUP, param3: *mut core::ffi::c_void) -> u32>;
pub type LPNODE_CALLBACK = Option<unsafe extern "system" fn(param0: HCLUSTER, param1: HNODE, param2: CLUSTER_NODE_STATE, param3: *mut core::ffi::c_void) -> u32>;
pub type LPRESOURCE_CALLBACK = Option<unsafe extern "system" fn(param0: HRESOURCE, param1: HRESOURCE, param2: *mut core::ffi::c_void) -> u32>;
pub type LPRESOURCE_CALLBACK_EX = Option<unsafe extern "system" fn(param0: HCLUSTER, param1: HRESOURCE, param2: HRESOURCE, param3: *mut core::ffi::c_void) -> u32>;
pub type PARBITRATE_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void, lostquorumresource: PQUORUM_RESOURCE_LOST) -> u32>;
pub type PARM_WPR_WATCHDOG_FOR_CURRENT_RESOURCE_CALL_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, timeoutinms: u64) -> u32>;
pub type PBEGIN_RESCALL_AS_USER_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void, tokenhandle: super::super::Foundation::HANDLE, controlcode: u32, inbuffer: *mut core::ffi::c_void, inbuffersize: u32, outbuffer: *mut core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut super::super::Foundation::BOOL) -> u32>;
pub type PBEGIN_RESCALL_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void, controlcode: u32, inbuffer: *mut core::ffi::c_void, inbuffersize: u32, outbuffer: *mut core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut super::super::Foundation::BOOL) -> u32>;
pub type PBEGIN_RESTYPECALL_AS_USER_ROUTINE = Option<unsafe extern "system" fn(resourcetypename: windows_core::PCWSTR, tokenhandle: super::super::Foundation::HANDLE, controlcode: u32, inbuffer: *mut core::ffi::c_void, inbuffersize: u32, outbuffer: *mut core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut super::super::Foundation::BOOL) -> u32>;
pub type PBEGIN_RESTYPECALL_ROUTINE = Option<unsafe extern "system" fn(resourcetypename: windows_core::PCWSTR, controlcode: u32, inbuffer: *mut core::ffi::c_void, inbuffersize: u32, outbuffer: *mut core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut super::super::Foundation::BOOL) -> u32>;
pub type PCANCEL_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void, cancelflags_reserved: u32) -> u32>;
pub type PCHANGE_RESOURCE_PROCESS_FOR_DUMPS = Option<unsafe extern "system" fn(resource: isize, processname: windows_core::PCWSTR, processid: u32, isadd: super::super::Foundation::BOOL) -> u32>;
pub type PCHANGE_RES_TYPE_PROCESS_FOR_DUMPS = Option<unsafe extern "system" fn(resourcetypename: windows_core::PCWSTR, processname: windows_core::PCWSTR, processid: u32, isadd: super::super::Foundation::BOOL) -> u32>;
pub type PCLOSE_CLUSTER_CRYPT_PROVIDER = Option<unsafe extern "system" fn(hcluscryptprovider: HCLUSCRYPTPROVIDER) -> u32>;
pub type PCLOSE_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void)>;
pub type PCLUSAPIClusWorkerCheckTerminate = Option<unsafe extern "system" fn(lpworker: *mut CLUS_WORKER) -> super::super::Foundation::BOOL>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_DEPENDENCY = Option<unsafe extern "system" fn(hdependentgroup: HGROUP, hprovidergroup: HGROUP) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_DEPENDENCY_EX = Option<unsafe extern "system" fn(hdependentgroup: HGROUP, hprovidergroup: HGROUP, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hdependentgroupset: HGROUPSET, hprovidergroupset: HGROUPSET) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EX = Option<unsafe extern "system" fn(hdependentgroupset: HGROUPSET, hprovidergroupset: HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hdependentgroup: HGROUP, hprovidergroupset: HGROUPSET) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY_EX = Option<unsafe extern "system" fn(hdependentgroup: HGROUP, hprovidergroupset: HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_NODE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznodename: windows_core::PCWSTR, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> HNODE>;
pub type PCLUSAPI_ADD_CLUSTER_NODE_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznodename: windows_core::PCWSTR, dwflags: u32, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> HNODE>;
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_DEPENDENCY = Option<unsafe extern "system" fn(hresource: HRESOURCE, hdependson: HRESOURCE) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_DEPENDENCY_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, hdependson: HRESOURCE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_NODE = Option<unsafe extern "system" fn(hresource: HRESOURCE, hnode: HNODE) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_NODE_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, hnode: HNODE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_CROSS_CLUSTER_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hdependentgroupset: HGROUPSET, lpremoteclustername: windows_core::PCWSTR, lpremotegroupsetname: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_RESOURCE_TO_CLUSTER_SHARED_VOLUMES = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> u32>;
pub type PCLUSAPI_BACKUP_CLUSTER_DATABASE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszpathname: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CAN_RESOURCE_BE_DEPENDENT = Option<unsafe extern "system" fn(hresource: HRESOURCE, hresourcedependent: HRESOURCE) -> super::super::Foundation::BOOL>;
pub type PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP = Option<unsafe extern "system" fn(hresource: HRESOURCE, hgroup: HGROUP) -> u32>;
pub type PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, hgroup: HGROUP, flags: u64) -> u32>;
pub type PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP_EX2 = Option<unsafe extern "system" fn(hresource: HRESOURCE, hgroup: HGROUP, flags: u64, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLOSE_CLUSTER = Option<unsafe extern "system" fn(hcluster: HCLUSTER) -> super::super::Foundation::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: HGROUP) -> super::super::Foundation::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_GROUP_GROUPSET = Option<unsafe extern "system" fn(hgroupset: HGROUPSET) -> super::super::Foundation::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_NETWORK = Option<unsafe extern "system" fn(hnetwork: HNETWORK) -> super::super::Foundation::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_NET_INTERFACE = Option<unsafe extern "system" fn(hnetinterface: HNETINTERFACE) -> super::super::Foundation::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_NODE = Option<unsafe extern "system" fn(hnode: HNODE) -> super::super::Foundation::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_NOTIFY_PORT = Option<unsafe extern "system" fn(hchange: HCHANGE) -> super::super::Foundation::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> super::super::Foundation::BOOL>;
pub type PCLUSAPI_CLUSTER_ADD_GROUP_TO_AFFINITY_RULE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, rulename: windows_core::PCWSTR, hgroup: HGROUP) -> u32>;
pub type PCLUSAPI_CLUSTER_ADD_GROUP_TO_GROUPSET_WITH_DOMAINS_EX = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, hgroup: HGROUP, faultdomain: u32, updatedomain: u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_ADD_GROUP_TO_GROUP_GROUPSET = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, hgroup: HGROUP) -> u32>;
pub type PCLUSAPI_CLUSTER_AFFINITY_RULE_CONTROL = Option<unsafe extern "system" fn(hcluster: HCLUSTER, affinityrulename: windows_core::PCWSTR, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_CLOSE_ENUM = Option<unsafe extern "system" fn(henum: HCLUSENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_CLOSE_ENUM_EX = Option<unsafe extern "system" fn(hclusterenum: HCLUSENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_CONTROL = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_CONTROL_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_CREATE_AFFINITY_RULE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, rulename: windows_core::PCWSTR, ruletype: CLUS_AFFINITY_RULE_TYPE) -> u32>;
pub type PCLUSAPI_CLUSTER_ENUM = Option<unsafe extern "system" fn(henum: HCLUSENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_ENUM_EX = Option<unsafe extern "system" fn(hclusterenum: HCLUSENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_GET_ENUM_COUNT = Option<unsafe extern "system" fn(henum: HCLUSENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_GET_ENUM_COUNT_EX = Option<unsafe extern "system" fn(hclusterenum: HCLUSENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_CLOSE_ENUM = Option<unsafe extern "system" fn(hgroupenum: HGROUPENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_CLOSE_ENUM_EX = Option<unsafe extern "system" fn(hgroupenumex: HGROUPENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_CONTROL = Option<unsafe extern "system" fn(hgroup: HGROUP, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_CONTROL_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_ENUM = Option<unsafe extern "system" fn(hgroupenum: HGROUPENUM, dwindex: u32, lpdwtype: *mut u32, lpszresourcename: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_ENUM_EX = Option<unsafe extern "system" fn(hgroupenumex: HGROUPENUMEX, dwindex: u32, pitem: *mut CLUSTER_GROUP_ENUM_ITEM, cbitem: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_GET_ENUM_COUNT = Option<unsafe extern "system" fn(hgroupenum: HGROUPENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_GET_ENUM_COUNT_EX = Option<unsafe extern "system" fn(hgroupenumex: HGROUPENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_GROUPSET_CONTROL = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_GROUPSET_CONTROL_EX = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_OPEN_ENUM = Option<unsafe extern "system" fn(hgroup: HGROUP, dwtype: u32) -> HGROUPENUM>;
pub type PCLUSAPI_CLUSTER_GROUP_OPEN_ENUM_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszproperties: windows_core::PCWSTR, cbproperties: u32, lpszroproperties: windows_core::PCWSTR, cbroproperties: u32, dwflags: u32) -> HGROUPENUMEX>;
pub type PCLUSAPI_CLUSTER_NETWORK_CLOSE_ENUM = Option<unsafe extern "system" fn(hnetworkenum: HNETWORKENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_NETWORK_CONTROL = Option<unsafe extern "system" fn(hnetwork: HNETWORK, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NETWORK_CONTROL_EX = Option<unsafe extern "system" fn(hnetwork: HNETWORK, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_NETWORK_ENUM = Option<unsafe extern "system" fn(hnetworkenum: HNETWORKENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NETWORK_GET_ENUM_COUNT = Option<unsafe extern "system" fn(hnetworkenum: HNETWORKENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_NETWORK_OPEN_ENUM = Option<unsafe extern "system" fn(hnetwork: HNETWORK, dwtype: u32) -> HNETWORKENUM>;
pub type PCLUSAPI_CLUSTER_NET_INTERFACE_CONTROL = Option<unsafe extern "system" fn(hnetinterface: HNETINTERFACE, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NET_INTERFACE_CONTROL_EX = Option<unsafe extern "system" fn(hnetinterface: HNETINTERFACE, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_CLOSE_ENUM = Option<unsafe extern "system" fn(hnodeenum: HNODEENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_CLOSE_ENUM_EX = Option<unsafe extern "system" fn(hnodeenum: HNODEENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_CONTROL = Option<unsafe extern "system" fn(hnode: HNODE, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_CONTROL_EX = Option<unsafe extern "system" fn(hnode: HNODE, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_ENUM = Option<unsafe extern "system" fn(hnodeenum: HNODEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_ENUM_EX = Option<unsafe extern "system" fn(hnodeenum: HNODEENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_GET_ENUM_COUNT = Option<unsafe extern "system" fn(hnodeenum: HNODEENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_GET_ENUM_COUNT_EX = Option<unsafe extern "system" fn(hnodeenum: HNODEENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_OPEN_ENUM = Option<unsafe extern "system" fn(hnode: HNODE, dwtype: u32) -> HNODEENUM>;
pub type PCLUSAPI_CLUSTER_NODE_OPEN_ENUM_EX = Option<unsafe extern "system" fn(hnode: HNODE, dwtype: u32, poptions: *const core::ffi::c_void) -> HNODEENUMEX>;
pub type PCLUSAPI_CLUSTER_OPEN_ENUM = Option<unsafe extern "system" fn(hcluster: HCLUSTER, dwtype: u32) -> HCLUSENUM>;
pub type PCLUSAPI_CLUSTER_OPEN_ENUM_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, dwtype: u32, poptions: *const core::ffi::c_void) -> HCLUSENUMEX>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_CLOSE_KEY = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_CREATE_BATCH = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, phregbatch: *mut HREGBATCH) -> i32>;
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_CREATE_KEY = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: windows_core::PCWSTR, dwoptions: u32, samdesired: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: *mut u32) -> i32>;
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_CREATE_KEY_EX = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: windows_core::PCWSTR, dwoptions: u32, samdesired: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: *mut u32, lpszreason: windows_core::PCWSTR) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_DELETE_KEY = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: windows_core::PCWSTR) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_DELETE_KEY_EX = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpsubkey: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_DELETE_VALUE = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: windows_core::PCWSTR) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_DELETE_VALUE_EX = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_ENUM_KEY = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, dwindex: u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_ENUM_VALUE = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, dwindex: u32, lpszvaluename: windows_core::PWSTR, lpcchvaluename: *mut u32, lpdwtype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> u32>;
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_GET_KEY_SECURITY = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, requestedinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_OPEN_KEY = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: windows_core::PCWSTR, samdesired: u32, phkresult: *mut super::super::System::Registry::HKEY) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_QUERY_INFO_KEY = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpcsubkeys: *mut u32, lpcbmaxsubkeylen: *mut u32, lpcvalues: *mut u32, lpcbmaxvaluenamelen: *mut u32, lpcbmaxvaluelen: *mut u32, lpcbsecuritydescriptor: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_QUERY_VALUE = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: windows_core::PCWSTR, lpdwvaluetype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> i32>;
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_SET_KEY_SECURITY = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> i32>;
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_SET_KEY_SECURITY_EX = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, lpszreason: windows_core::PCWSTR) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_SET_VALUE = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: windows_core::PCWSTR, dwtype: u32, lpdata: *const u8, cbdata: u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_SET_VALUE_EX = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: windows_core::PCWSTR, dwtype: u32, lpdata: *const u8, cbdata: u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_REG_SYNC_DATABASE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, flags: u32) -> i32>;
pub type PCLUSAPI_CLUSTER_REMOVE_AFFINITY_RULE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, rulename: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_AFFINITY_RULE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, rulename: windows_core::PCWSTR, hgroup: HGROUP) -> u32>;
pub type PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_GROUPSET = Option<unsafe extern "system" fn(hgroupset: HGROUPSET) -> u32>;
pub type PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_GROUPSET_EX = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_CLOSE_ENUM = Option<unsafe extern "system" fn(hresenum: HRESENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_CLOSE_ENUM_EX = Option<unsafe extern "system" fn(hresourceenumex: HRESENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_CONTROL = Option<unsafe extern "system" fn(hresource: HRESOURCE, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_CONTROL_AS_USER_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_CONTROL_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_ENUM = Option<unsafe extern "system" fn(hresenum: HRESENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_ENUM_EX = Option<unsafe extern "system" fn(hresourceenumex: HRESENUMEX, dwindex: u32, pitem: *mut CLUSTER_RESOURCE_ENUM_ITEM, cbitem: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_GET_ENUM_COUNT = Option<unsafe extern "system" fn(hresenum: HRESENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_GET_ENUM_COUNT_EX = Option<unsafe extern "system" fn(hresourceenumex: HRESENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_OPEN_ENUM = Option<unsafe extern "system" fn(hresource: HRESOURCE, dwtype: u32) -> HRESENUM>;
pub type PCLUSAPI_CLUSTER_RESOURCE_OPEN_ENUM_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszproperties: windows_core::PCWSTR, cbproperties: u32, lpszroproperties: windows_core::PCWSTR, cbroproperties: u32, dwflags: u32) -> HRESENUMEX>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CLOSE_ENUM = Option<unsafe extern "system" fn(hrestypeenum: HRESTYPEENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CONTROL = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CONTROL_AS_USER_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CONTROL_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_ENUM = Option<unsafe extern "system" fn(hrestypeenum: HRESTYPEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_GET_ENUM_COUNT = Option<unsafe extern "system" fn(hrestypeenum: HRESTYPEENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_OPEN_ENUM = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, dwtype: u32) -> HRESTYPEENUM>;
pub type PCLUSAPI_CLUSTER_UPGRADE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, perform: super::super::Foundation::BOOL, pfnprogresscallback: PCLUSTER_UPGRADE_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> u32>;
pub type PCLUSAPI_CLUS_WORKER_CREATE = Option<unsafe extern "system" fn(lpworker: *mut CLUS_WORKER, lpstartaddress: PWORKER_START_ROUTINE, lpparameter: *mut core::ffi::c_void) -> u32>;
pub type PCLUSAPI_CLUS_WORKER_TERMINATE = Option<unsafe extern "system" fn(lpworker: *const CLUS_WORKER)>;
pub type PCLUSAPI_CREATE_CLUSTER = Option<unsafe extern "system" fn(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> HCLUSTER>;
pub type PCLUSAPI_CREATE_CLUSTER_AVAILABILITY_SET = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpavailabilitysetname: windows_core::PCWSTR, pavailabilitysetconfig: *const CLUSTER_AVAILABILITY_SET_CONFIG) -> HGROUPSET>;
pub type PCLUSAPI_CREATE_CLUSTER_CNOLESS = Option<unsafe extern "system" fn(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> HCLUSTER>;
pub type PCLUSAPI_CREATE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszgroupname: windows_core::PCWSTR) -> HGROUP>;
pub type PCLUSAPI_CREATE_CLUSTER_GROUPEX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszgroupname: windows_core::PCWSTR, pgroupinfo: *const CLUSTER_CREATE_GROUP_INFO) -> HGROUP>;
pub type PCLUSAPI_CREATE_CLUSTER_GROUP_GROUPSET = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszgroupsetname: windows_core::PCWSTR) -> HGROUPSET>;
pub type PCLUSAPI_CREATE_CLUSTER_NAME_ACCOUNT = Option<unsafe extern "system" fn(hcluster: HCLUSTER, pconfig: *const CREATE_CLUSTER_NAME_ACCOUNT, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> u32>;
pub type PCLUSAPI_CREATE_CLUSTER_NOTIFY_PORT = Option<unsafe extern "system" fn(hchange: HCHANGE, hcluster: HCLUSTER, dwfilter: u32, dwnotifykey: usize) -> HCHANGE>;
pub type PCLUSAPI_CREATE_CLUSTER_NOTIFY_PORT_V2 = Option<unsafe extern "system" fn(hchange: HCHANGE, hcluster: HCLUSTER, filters: *const NOTIFY_FILTER_AND_TYPE, dwfiltercount: u32, dwnotifykey: usize) -> HCHANGE>;
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hgroup: HGROUP, lpszresourcename: windows_core::PCWSTR, lpszresourcetype: windows_core::PCWSTR, dwflags: u32) -> HRESOURCE>;
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, lpszresourcename: windows_core::PCWSTR, lpszresourcetype: windows_core::PCWSTR, dwflags: u32, lpszreason: windows_core::PCWSTR) -> HRESOURCE>;
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE_TYPE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, lpszdisplayname: windows_core::PCWSTR, lpszresourcetypedll: windows_core::PCWSTR, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32) -> u32>;
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE_TYPE_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, lpszdisplayname: windows_core::PCWSTR, lpszresourcetypedll: windows_core::PCWSTR, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: HGROUP) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_GROUP_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_GROUP_GROUPSET = Option<unsafe extern "system" fn(hgroupset: HGROUPSET) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_GROUP_GROUPSET_EX = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE_TYPE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcetypename: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE_TYPE_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsztypename: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DESTROY_CLUSTER = Option<unsafe extern "system" fn(hcluster: HCLUSTER, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void, fdeletevirtualcomputerobjects: super::super::Foundation::BOOL) -> u32>;
pub type PCLUSAPI_DESTROY_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: HGROUP) -> u32>;
pub type PCLUSAPI_DESTROY_CLUSTER_GROUP_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_EVICT_CLUSTER_NODE = Option<unsafe extern "system" fn(hnode: HNODE) -> u32>;
pub type PCLUSAPI_EVICT_CLUSTER_NODE_EX = Option<unsafe extern "system" fn(hnode: HNODE, dwtimeout: u32, phrcleanupstatus: *mut windows_core::HRESULT) -> u32>;
pub type PCLUSAPI_EVICT_CLUSTER_NODE_EX2 = Option<unsafe extern "system" fn(hnode: HNODE, dwtimeout: u32, phrcleanupstatus: *mut windows_core::HRESULT, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_FAIL_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> u32>;
pub type PCLUSAPI_FAIL_CLUSTER_RESOURCE_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_GET_CLUSTER_FROM_GROUP = Option<unsafe extern "system" fn(hgroup: HGROUP) -> HCLUSTER>;
pub type PCLUSAPI_GET_CLUSTER_FROM_GROUP_GROUPSET = Option<unsafe extern "system" fn(hgroupset: HGROUPSET) -> HCLUSTER>;
pub type PCLUSAPI_GET_CLUSTER_FROM_NETWORK = Option<unsafe extern "system" fn(hnetwork: HNETWORK) -> HCLUSTER>;
pub type PCLUSAPI_GET_CLUSTER_FROM_NET_INTERFACE = Option<unsafe extern "system" fn(hnetinterface: HNETINTERFACE) -> HCLUSTER>;
pub type PCLUSAPI_GET_CLUSTER_FROM_NODE = Option<unsafe extern "system" fn(hnode: HNODE) -> HCLUSTER>;
pub type PCLUSAPI_GET_CLUSTER_FROM_RESOURCE = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> HCLUSTER>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_GROUP_KEY = Option<unsafe extern "system" fn(hgroup: HGROUP, samdesired: u32) -> super::super::System::Registry::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_GROUP_STATE = Option<unsafe extern "system" fn(hgroup: HGROUP, lpsznodename: windows_core::PWSTR, lpcchnodename: *mut u32) -> CLUSTER_GROUP_STATE>;
pub type PCLUSAPI_GET_CLUSTER_INFORMATION = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszclustername: windows_core::PWSTR, lpcchclustername: *mut u32, lpclusterinfo: *mut CLUSTERVERSIONINFO) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_KEY = Option<unsafe extern "system" fn(hcluster: HCLUSTER, samdesired: u32) -> super::super::System::Registry::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_NETWORK_ID = Option<unsafe extern "system" fn(hnetwork: HNETWORK, lpsznetworkid: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_NETWORK_KEY = Option<unsafe extern "system" fn(hnetwork: HNETWORK, samdesired: u32) -> super::super::System::Registry::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_NETWORK_STATE = Option<unsafe extern "system" fn(hnetwork: HNETWORK) -> CLUSTER_NETWORK_STATE>;
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznodename: windows_core::PCWSTR, lpsznetworkname: windows_core::PCWSTR, lpszinterfacename: windows_core::PWSTR, lpcchinterfacename: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE_KEY = Option<unsafe extern "system" fn(hnetinterface: HNETINTERFACE, samdesired: u32) -> super::super::System::Registry::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE_STATE = Option<unsafe extern "system" fn(hnetinterface: HNETINTERFACE) -> CLUSTER_NETINTERFACE_STATE>;
pub type PCLUSAPI_GET_CLUSTER_NODE_ID = Option<unsafe extern "system" fn(hnode: HNODE, lpsznodeid: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_NODE_KEY = Option<unsafe extern "system" fn(hnode: HNODE, samdesired: u32) -> super::super::System::Registry::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_NODE_STATE = Option<unsafe extern "system" fn(hnode: HNODE) -> CLUSTER_NODE_STATE>;
pub type PCLUSAPI_GET_CLUSTER_NOTIFY = Option<unsafe extern "system" fn(hchange: HCHANGE, lpdwnotifykey: *mut usize, lpdwfiltertype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32, dwmilliseconds: u32) -> u32>;
pub type PCLUSAPI_GET_CLUSTER_NOTIFY_V2 = Option<unsafe extern "system" fn(hchange: HCHANGE, lpdwnotifykey: *mut usize, pfilterandtype: *mut NOTIFY_FILTER_AND_TYPE, buffer: *mut u8, lpcchbuffersize: *mut u32, lpszobjectid: windows_core::PWSTR, lpcchobjectid: *mut u32, lpszparentid: windows_core::PWSTR, lpcchparentid: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32, lpsztype: windows_core::PWSTR, lpcchtype: *mut u32, dwmilliseconds: u32) -> u32>;
pub type PCLUSAPI_GET_CLUSTER_QUORUM_RESOURCE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcename: windows_core::PWSTR, lpcchresourcename: *mut u32, lpszdevicename: windows_core::PWSTR, lpcchdevicename: *mut u32, lpdwmaxquorumlogsize: *mut u32) -> u32>;
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_DEPENDENCY_EXPRESSION = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpszdependencyexpression: windows_core::PWSTR, lpcchdependencyexpression: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_KEY = Option<unsafe extern "system" fn(hresource: HRESOURCE, samdesired: u32) -> super::super::System::Registry::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_NETWORK_NAME = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpbuffer: windows_core::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL>;
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_STATE = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpsznodename: windows_core::PWSTR, lpcchnodename: *mut u32, lpszgroupname: windows_core::PWSTR, lpcchgroupname: *mut u32) -> CLUSTER_RESOURCE_STATE>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_TYPE_KEY = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsztypename: windows_core::PCWSTR, samdesired: u32) -> super::super::System::Registry::HKEY>;
pub type PCLUSAPI_GET_NODE_CLUSTER_STATE = Option<unsafe extern "system" fn(lpsznodename: windows_core::PCWSTR, pdwclusterstate: *mut u32) -> u32>;
pub type PCLUSAPI_GET_NOTIFY_EVENT_HANDLE_V2 = Option<unsafe extern "system" fn(hchange: HCHANGE, lphtargetevent: *mut super::super::Foundation::HANDLE) -> u32>;
pub type PCLUSAPI_IS_FILE_ON_CLUSTER_SHARED_VOLUME = Option<unsafe extern "system" fn(lpszpathname: windows_core::PCWSTR, pbfileisonsharedvolume: *mut super::super::Foundation::BOOL) -> u32>;
pub type PCLUSAPI_MOVE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: HGROUP, hdestinationnode: HNODE) -> u32>;
pub type PCLUSAPI_OFFLINE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: HGROUP) -> u32>;
pub type PCLUSAPI_OFFLINE_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> u32>;
pub type PCLUSAPI_ONLINE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: HGROUP, hdestinationnode: HNODE) -> u32>;
pub type PCLUSAPI_ONLINE_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> u32>;
pub type PCLUSAPI_OPEN_CLUSTER = Option<unsafe extern "system" fn(lpszclustername: windows_core::PCWSTR) -> HCLUSTER>;
pub type PCLUSAPI_OPEN_CLUSTER_EX = Option<unsafe extern "system" fn(lpszclustername: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HCLUSTER>;
pub type PCLUSAPI_OPEN_CLUSTER_GROUP = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszgroupname: windows_core::PCWSTR) -> HGROUP>;
pub type PCLUSAPI_OPEN_CLUSTER_GROUP_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszgroupname: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HGROUP>;
pub type PCLUSAPI_OPEN_CLUSTER_GROUP_GROUPSET = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszgroupsetname: windows_core::PCWSTR) -> HGROUPSET>;
pub type PCLUSAPI_OPEN_CLUSTER_NETINTERFACE_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznetinterfacename: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HNETINTERFACE>;
pub type PCLUSAPI_OPEN_CLUSTER_NETWORK = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznetworkname: windows_core::PCWSTR) -> HNETWORK>;
pub type PCLUSAPI_OPEN_CLUSTER_NETWORK_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznetworkname: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HNETWORK>;
pub type PCLUSAPI_OPEN_CLUSTER_NET_INTERFACE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszinterfacename: windows_core::PCWSTR) -> HNETINTERFACE>;
pub type PCLUSAPI_OPEN_CLUSTER_NODE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznodename: windows_core::PCWSTR) -> HNODE>;
pub type PCLUSAPI_OPEN_CLUSTER_NODE_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznodename: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HNODE>;
pub type PCLUSAPI_OPEN_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcename: windows_core::PCWSTR) -> HRESOURCE>;
pub type PCLUSAPI_OPEN_CLUSTER_RESOURCE_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcename: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HRESOURCE>;
pub type PCLUSAPI_OPEN_NODE_BY_ID = Option<unsafe extern "system" fn(hcluster: HCLUSTER, nodeid: u32) -> HNODE>;
pub type PCLUSAPI_PAUSE_CLUSTER_NODE = Option<unsafe extern "system" fn(hnode: HNODE) -> u32>;
pub type PCLUSAPI_PAUSE_CLUSTER_NODE_EX = Option<unsafe extern "system" fn(hnode: HNODE, bdrainnode: super::super::Foundation::BOOL, dwpauseflags: u32, hnodedraintarget: HNODE) -> u32>;
pub type PCLUSAPI_PAUSE_CLUSTER_NODE_EX2 = Option<unsafe extern "system" fn(hnode: HNODE, bdrainnode: super::super::Foundation::BOOL, dwpauseflags: u32, hnodedraintarget: HNODE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_PFN_REASON_HANDLER = Option<unsafe extern "system" fn(lpparameter: *const core::ffi::c_void, hcluster: HCLUSTER, szreason: windows_core::PWSTR, lpsize: *mut u32) -> super::super::Foundation::BOOL>;
pub type PCLUSAPI_REGISTER_CLUSTER_NOTIFY = Option<unsafe extern "system" fn(hchange: HCHANGE, dwfiltertype: u32, hobject: super::super::Foundation::HANDLE, dwnotifykey: usize) -> u32>;
pub type PCLUSAPI_REGISTER_CLUSTER_NOTIFY_V2 = Option<unsafe extern "system" fn(hchange: HCHANGE, filter: NOTIFY_FILTER_AND_TYPE, hobject: super::super::Foundation::HANDLE, dwnotifykey: usize) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_DEPENDENCY = Option<unsafe extern "system" fn(hgroup: HGROUP, hdependson: HGROUP) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_DEPENDENCY_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, hdependson: HGROUP, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, hdependson: HGROUPSET) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EX = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, hdependson: HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hgroup: HGROUP, hdependson: HGROUPSET) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, hdependson: HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_NAME_ACCOUNT = Option<unsafe extern "system" fn(hcluster: HCLUSTER) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_DEPENDENCY = Option<unsafe extern "system" fn(hresource: HRESOURCE, hdependson: HRESOURCE) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_DEPENDENCY_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, hdependson: HRESOURCE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_NODE = Option<unsafe extern "system" fn(hresource: HRESOURCE, hnode: HNODE) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_NODE_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, hnode: HNODE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_CROSS_CLUSTER_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hdependentgroupset: HGROUPSET, lpremoteclustername: windows_core::PCWSTR, lpremotegroupsetname: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_RESOURCE_FROM_CLUSTER_SHARED_VOLUMES = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> u32>;
pub type PCLUSAPI_RESTART_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: HRESOURCE, dwflags: u32) -> u32>;
pub type PCLUSAPI_RESTART_CLUSTER_RESOURCE_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, dwflags: u32) -> u32>;
pub type PCLUSAPI_RESTORE_CLUSTER_DATABASE = Option<unsafe extern "system" fn(lpszpathname: windows_core::PCWSTR, bforce: super::super::Foundation::BOOL, lpszquorumdriveletter: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_RESUME_CLUSTER_NODE = Option<unsafe extern "system" fn(hnode: HNODE) -> u32>;
pub type PCLUSAPI_RESUME_CLUSTER_NODE_EX = Option<unsafe extern "system" fn(hnode: HNODE, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32) -> u32>;
pub type PCLUSAPI_RESUME_CLUSTER_NODE_EX2 = Option<unsafe extern "system" fn(hnode: HNODE, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EXPRESSION = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, lpszdependencyexpression: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EXPRESSION_EX = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, lpszdependencyexpression: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_NAME = Option<unsafe extern "system" fn(hgroup: HGROUP, lpszgroupname: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_NAME_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, lpszgroupname: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_NODE_LIST = Option<unsafe extern "system" fn(hgroup: HGROUP, nodecount: u32, nodelist: *const HNODE) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_NODE_LIST_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, nodecount: u32, nodelist: *const HNODE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_NAME_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznewclustername: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_NETWORK_NAME = Option<unsafe extern "system" fn(hnetwork: HNETWORK, lpszname: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_NETWORK_NAME_EX = Option<unsafe extern "system" fn(hnetwork: HNETWORK, lpszname: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_NETWORK_PRIORITY_ORDER = Option<unsafe extern "system" fn(hcluster: HCLUSTER, networkcount: u32, networklist: *const HNETWORK) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_QUORUM_RESOURCE = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpszdevicename: windows_core::PCWSTR, dwmaxquologsize: u32) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_QUORUM_RESOURCE_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpszdevicename: windows_core::PCWSTR, dwmaxquorumlogsize: u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_RESOURCE_DEPENDENCY_EXPRESSION = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpszdependencyexpression: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_RESOURCE_NAME = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpszresourcename: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_RESOURCE_NAME_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpszresourcename: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_SERVICE_ACCOUNT_PASSWORD = Option<unsafe extern "system" fn(lpszclustername: windows_core::PCWSTR, lpsznewpassword: windows_core::PCWSTR, dwflags: u32, lpreturnstatusbuffer: *mut CLUSTER_SET_PASSWORD_STATUS, lpcbreturnstatusbuffersize: *mut u32) -> u32>;
pub type PCLUSAPI_SET_GROUP_DEPENDENCY_EXPRESSION = Option<unsafe extern "system" fn(hgroupset: HGROUP, lpszdependencyexpression: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_GROUP_DEPENDENCY_EXPRESSION_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, lpszdependencyexpression: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_REASON_HANDLER = Option<unsafe extern "system" fn(lphandler: *const CLUSAPI_REASON_HANDLER) -> *mut CLUSAPI_REASON_HANDLER>;
pub type PCLUSAPI_SHARED_VOLUME_SET_SNAPSHOT_STATE = Option<unsafe extern "system" fn(guidsnapshotset: windows_core::GUID, lpszvolumename: windows_core::PCWSTR, state: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE) -> u32>;
pub type PCLUSAPI_SetClusterName = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznewclustername: windows_core::PCWSTR) -> u32>;
pub type PCLUSTER_CLEAR_BACKUP_STATE_FOR_SHARED_VOLUME = Option<unsafe extern "system" fn(lpszvolumepathname: windows_core::PCWSTR) -> u32>;
pub type PCLUSTER_DECRYPT = Option<unsafe extern "system" fn(hcluscryptprovider: HCLUSCRYPTPROVIDER, pcryptinput: *const u8, cbcryptinput: u32, ppcryptoutput: *mut *mut u8, pcbcryptoutput: *mut u32) -> u32>;
pub type PCLUSTER_ENCRYPT = Option<unsafe extern "system" fn(hcluscryptprovider: HCLUSCRYPTPROVIDER, pdata: *const u8, cbdata: u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> u32>;
pub type PCLUSTER_GET_VOLUME_NAME_FOR_VOLUME_MOUNT_POINT = Option<unsafe extern "system" fn(lpszvolumemountpoint: windows_core::PCWSTR, lpszvolumename: windows_core::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL>;
pub type PCLUSTER_GET_VOLUME_PATH_NAME = Option<unsafe extern "system" fn(lpszfilename: windows_core::PCWSTR, lpszvolumepathname: windows_core::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL>;
pub type PCLUSTER_IS_PATH_ON_SHARED_VOLUME = Option<unsafe extern "system" fn(lpszpathname: windows_core::PCWSTR) -> super::super::Foundation::BOOL>;
pub type PCLUSTER_PREPARE_SHARED_VOLUME_FOR_BACKUP = Option<unsafe extern "system" fn(lpszfilename: windows_core::PCWSTR, lpszvolumepathname: windows_core::PWSTR, lpcchvolumepathname: *mut u32, lpszvolumename: windows_core::PWSTR, lpcchvolumename: *mut u32) -> u32>;
pub type PCLUSTER_REG_BATCH_ADD_COMMAND = Option<unsafe extern "system" fn(hregbatch: HREGBATCH, dwcommand: CLUSTER_REG_COMMAND, wzname: windows_core::PCWSTR, dwoptions: u32, lpdata: *const core::ffi::c_void, cbdata: u32) -> i32>;
pub type PCLUSTER_REG_BATCH_CLOSE_NOTIFICATION = Option<unsafe extern "system" fn(hbatchnotification: HREGBATCHNOTIFICATION) -> i32>;
pub type PCLUSTER_REG_BATCH_READ_COMMAND = Option<unsafe extern "system" fn(hbatchnotification: HREGBATCHNOTIFICATION, pbatchcommand: *mut CLUSTER_BATCH_COMMAND) -> i32>;
pub type PCLUSTER_REG_CLOSE_BATCH = Option<unsafe extern "system" fn(hregbatch: HREGBATCH, bcommit: super::super::Foundation::BOOL, failedcommandnumber: *mut i32) -> i32>;
pub type PCLUSTER_REG_CLOSE_BATCH_NOTIFY_PORT = Option<unsafe extern "system" fn(hbatchnotifyport: HREGBATCHPORT) -> i32>;
pub type PCLUSTER_REG_CLOSE_READ_BATCH = Option<unsafe extern "system" fn(hregreadbatch: HREGREADBATCH, phregreadbatchreply: *mut HREGREADBATCHREPLY) -> i32>;
pub type PCLUSTER_REG_CLOSE_READ_BATCH_EX = Option<unsafe extern "system" fn(hregreadbatch: HREGREADBATCH, flags: u32, phregreadbatchreply: *mut HREGREADBATCHREPLY) -> i32>;
pub type PCLUSTER_REG_CLOSE_READ_BATCH_REPLY = Option<unsafe extern "system" fn(hregreadbatchreply: HREGREADBATCHREPLY) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSTER_REG_CREATE_BATCH_NOTIFY_PORT = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, phbatchnotifyport: *mut HREGBATCHPORT) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSTER_REG_CREATE_READ_BATCH = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, phregreadbatch: *mut HREGREADBATCH) -> i32>;
pub type PCLUSTER_REG_GET_BATCH_NOTIFICATION = Option<unsafe extern "system" fn(hbatchnotify: HREGBATCHPORT, phbatchnotification: *mut HREGBATCHNOTIFICATION) -> i32>;
pub type PCLUSTER_REG_READ_BATCH_ADD_COMMAND = Option<unsafe extern "system" fn(hregreadbatch: HREGREADBATCH, wzsubkeyname: windows_core::PCWSTR, wzvaluename: windows_core::PCWSTR) -> i32>;
pub type PCLUSTER_REG_READ_BATCH_REPLY_NEXT_COMMAND = Option<unsafe extern "system" fn(hregreadbatchreply: HREGREADBATCHREPLY, pbatchcommand: *mut CLUSTER_READ_BATCH_COMMAND) -> i32>;
pub type PCLUSTER_SETUP_PROGRESS_CALLBACK = Option<unsafe extern "system" fn(pvcallbackarg: *mut core::ffi::c_void, esetupphase: CLUSTER_SETUP_PHASE, ephasetype: CLUSTER_SETUP_PHASE_TYPE, ephaseseverity: CLUSTER_SETUP_PHASE_SEVERITY, dwpercentcomplete: u32, lpszobjectname: windows_core::PCWSTR, dwstatus: u32) -> super::super::Foundation::BOOL>;
pub type PCLUSTER_SET_ACCOUNT_ACCESS = Option<unsafe extern "system" fn(hcluster: HCLUSTER, szaccountsid: windows_core::PCWSTR, dwaccess: u32, dwcontroltype: u32) -> u32>;
pub type PCLUSTER_UPGRADE_PROGRESS_CALLBACK = Option<unsafe extern "system" fn(pvcallbackarg: *mut core::ffi::c_void, eupgradephase: CLUSTER_UPGRADE_PHASE) -> super::super::Foundation::BOOL>;
pub type PEND_CONTROL_CALL = Option<unsafe extern "system" fn(context: i64, status: u32) -> u32>;
pub type PEND_TYPE_CONTROL_CALL = Option<unsafe extern "system" fn(context: i64, status: u32) -> u32>;
pub type PEXTEND_RES_CONTROL_CALL = Option<unsafe extern "system" fn(context: i64, newtimeoutinms: u32) -> u32>;
pub type PEXTEND_RES_TYPE_CONTROL_CALL = Option<unsafe extern "system" fn(context: i64, newtimeoutinms: u32) -> u32>;
pub type PFREE_CLUSTER_CRYPT = Option<unsafe extern "system" fn(pcryptinfo: *const core::ffi::c_void) -> u32>;
pub type PIS_ALIVE_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type PLOG_EVENT_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, loglevel: LOG_LEVEL, formatstring: windows_core::PCWSTR)>;
pub type PLOOKS_ALIVE_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type POFFLINE_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void) -> u32>;
pub type POFFLINE_V2_ROUTINE = Option<unsafe extern "system" fn(resource: *const core::ffi::c_void, destinationnodename: windows_core::PCWSTR, offlineflags: u32, inbuffer: *const u8, inbuffersize: u32, reserved: u32) -> u32>;
pub type PONLINE_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void, eventhandle: *mut super::super::Foundation::HANDLE) -> u32>;
pub type PONLINE_V2_ROUTINE = Option<unsafe extern "system" fn(resource: *const core::ffi::c_void, eventhandle: *mut super::super::Foundation::HANDLE, onlineflags: u32, inbuffer: *const u8, inbuffersize: u32, reserved: u32) -> u32>;
pub type POPEN_CLUSTER_CRYPT_PROVIDER = Option<unsafe extern "system" fn(lpszresource: windows_core::PCWSTR, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> HCLUSCRYPTPROVIDER>;
pub type POPEN_CLUSTER_CRYPT_PROVIDEREX = Option<unsafe extern "system" fn(lpszresource: windows_core::PCWSTR, lpszkeyname: windows_core::PCWSTR, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> HCLUSCRYPTPROVIDER>;
#[cfg(feature = "Win32_System_Registry")]
pub type POPEN_ROUTINE = Option<unsafe extern "system" fn(resourcename: windows_core::PCWSTR, resourcekey: super::super::System::Registry::HKEY, resourcehandle: isize) -> *mut core::ffi::c_void>;
#[cfg(feature = "Win32_System_Registry")]
pub type POPEN_V2_ROUTINE = Option<unsafe extern "system" fn(resourcename: windows_core::PCWSTR, resourcekey: super::super::System::Registry::HKEY, resourcehandle: isize, openflags: u32) -> *mut core::ffi::c_void>;
pub type PQUERY_APPINSTANCE_VERSION = Option<unsafe extern "system" fn(appinstanceid: *const windows_core::GUID, instanceversionhigh: *mut u64, instanceversionlow: *mut u64, versionstatus: *mut super::super::Foundation::NTSTATUS) -> u32>;
pub type PQUORUM_RESOURCE_LOST = Option<unsafe extern "system" fn(resource: isize)>;
pub type PRAISE_RES_TYPE_NOTIFICATION = Option<unsafe extern "system" fn(resourcetype: windows_core::PCWSTR, ppayload: *const u8, payloadsize: u32) -> u32>;
pub type PREGISTER_APPINSTANCE = Option<unsafe extern "system" fn(processhandle: super::super::Foundation::HANDLE, appinstanceid: *const windows_core::GUID, childreninheritappinstance: super::super::Foundation::BOOL) -> u32>;
pub type PREGISTER_APPINSTANCE_VERSION = Option<unsafe extern "system" fn(appinstanceid: *const windows_core::GUID, instanceversionhigh: u64, instanceversionlow: u64) -> u32>;
pub type PRELEASE_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void) -> u32>;
pub type PREQUEST_DUMP_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, dumpduetocallinprogress: super::super::Foundation::BOOL, dumpdelayinms: u32) -> u32>;
pub type PRESET_ALL_APPINSTANCE_VERSIONS = Option<unsafe extern "system" fn() -> u32>;
pub type PRESOURCE_CONTROL_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void, controlcode: u32, inbuffer: *mut core::ffi::c_void, inbuffersize: u32, outbuffer: *mut core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32) -> u32>;
pub type PRESOURCE_TYPE_CONTROL_ROUTINE = Option<unsafe extern "system" fn(resourcetypename: windows_core::PCWSTR, controlcode: u32, inbuffer: *mut core::ffi::c_void, inbuffersize: u32, outbuffer: *mut core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_ADD_UNKNOWN_PROPERTIES = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut core::ffi::c_void, pcboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
pub type PRESUTIL_CREATE_DIRECTORY_TREE = Option<unsafe extern "system" fn(pszpath: windows_core::PCWSTR) -> u32>;
pub type PRESUTIL_DUP_PARAMETER_BLOCK = Option<unsafe extern "system" fn(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM) -> u32>;
pub type PRESUTIL_DUP_STRING = Option<unsafe extern "system" fn(pszinstring: windows_core::PCWSTR) -> windows_core::PWSTR>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_ENUM_PRIVATE_PROPERTIES = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszoutproperties: windows_core::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
pub type PRESUTIL_ENUM_PROPERTIES = Option<unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, pszoutproperties: windows_core::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
pub type PRESUTIL_ENUM_RESOURCES = Option<unsafe extern "system" fn(hself: HRESOURCE, lpszrestypename: windows_core::PCWSTR, prescallback: LPRESOURCE_CALLBACK, pparameter: *mut core::ffi::c_void) -> u32>;
pub type PRESUTIL_ENUM_RESOURCES_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hself: HRESOURCE, lpszrestypename: windows_core::PCWSTR, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut core::ffi::c_void) -> u32>;
pub type PRESUTIL_ENUM_RESOURCES_EX2 = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hself: HRESOURCE, lpszrestypename: windows_core::PCWSTR, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut core::ffi::c_void, dwdesiredaccess: u32) -> u32>;
pub type PRESUTIL_EXPAND_ENVIRONMENT_STRINGS = Option<unsafe extern "system" fn(pszsrc: windows_core::PCWSTR) -> windows_core::PWSTR>;
pub type PRESUTIL_FIND_BINARY_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, pbpropertyvalue: *mut *mut u8, pcbpropertyvaluesize: *mut u32) -> u32>;
pub type PRESUTIL_FIND_DEPENDENT_DISK_RESOURCE_DRIVE_LETTER = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hresource: HRESOURCE, pszdriveletter: windows_core::PWSTR, pcchdriveletter: *mut u32) -> u32>;
pub type PRESUTIL_FIND_DWORD_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, pdwpropertyvalue: *mut u32) -> u32>;
pub type PRESUTIL_FIND_EXPANDED_SZ_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, pszpropertyvalue: *mut windows_core::PWSTR) -> u32>;
pub type PRESUTIL_FIND_EXPAND_SZ_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, pszpropertyvalue: *mut windows_core::PWSTR) -> u32>;
pub type PRESUTIL_FIND_FILETIME_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, pftpropertyvalue: *mut super::super::Foundation::FILETIME) -> u32>;
pub type PRESUTIL_FIND_LONG_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, plpropertyvalue: *mut i32) -> u32>;
pub type PRESUTIL_FIND_MULTI_SZ_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, pszpropertyvalue: *mut windows_core::PWSTR, pcbpropertyvaluesize: *mut u32) -> u32>;
pub type PRESUTIL_FIND_SZ_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, pszpropertyvalue: *mut windows_core::PWSTR) -> u32>;
pub type PRESUTIL_FIND_ULARGEINTEGER_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, plpropertyvalue: *mut u64) -> u32>;
pub type PRESUTIL_FREE_ENVIRONMENT = Option<unsafe extern "system" fn(lpenvironment: *mut core::ffi::c_void) -> u32>;
pub type PRESUTIL_FREE_PARAMETER_BLOCK = Option<unsafe extern "system" fn(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM)>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_ALL_PROPERTIES = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
pub type PRESUTIL_GET_BINARY_PROPERTY = Option<unsafe extern "system" fn(ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_BINARY, pboldvalue: *const u8, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_BINARY_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32>;
pub type PRESUTIL_GET_CORE_CLUSTER_RESOURCES = Option<unsafe extern "system" fn(hcluster: HCLUSTER, phclusternameresource: *mut HRESOURCE, phclusteripaddressresource: *mut HRESOURCE, phclusterquorumresource: *mut HRESOURCE) -> u32>;
pub type PRESUTIL_GET_CORE_CLUSTER_RESOURCES_EX = Option<unsafe extern "system" fn(hclusterin: HCLUSTER, phclusternameresourceout: *mut HRESOURCE, phclusteripaddressresourceout: *mut HRESOURCE, phclusterquorumresourceout: *mut HRESOURCE, dwdesiredaccess: u32) -> u32>;
pub type PRESUTIL_GET_DWORD_PROPERTY = Option<unsafe extern "system" fn(pdwoutvalue: *mut u32, pvaluestruct: *const CLUSPROP_DWORD, dwoldvalue: u32, dwminimum: u32, dwmaximum: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_DWORD_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, pdwoutvalue: *mut u32, dwdefaultvalue: u32) -> u32>;
pub type PRESUTIL_GET_ENVIRONMENT_WITH_NET_NAME = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> *mut core::ffi::c_void>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_EXPAND_SZ_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, bexpand: super::super::Foundation::BOOL) -> windows_core::PWSTR>;
pub type PRESUTIL_GET_FILETIME_PROPERTY = Option<unsafe extern "system" fn(pftoutvalue: *mut super::super::Foundation::FILETIME, pvaluestruct: *const CLUSPROP_FILETIME, ftoldvalue: super::super::Foundation::FILETIME, ftminimum: super::super::Foundation::FILETIME, ftmaximum: super::super::Foundation::FILETIME, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
pub type PRESUTIL_GET_LONG_PROPERTY = Option<unsafe extern "system" fn(ploutvalue: *mut i32, pvaluestruct: *const CLUSPROP_LONG, loldvalue: i32, lminimum: i32, lmaximum: i32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
pub type PRESUTIL_GET_MULTI_SZ_PROPERTY = Option<unsafe extern "system" fn(ppszoutvalue: *mut windows_core::PWSTR, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: windows_core::PCWSTR, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_PRIVATE_PROPERTIES = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, poutpropertylist: *mut core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_PROPERTIES = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_PROPERTIES_TO_PARAMETER_BLOCK = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutparams: *mut u8, bcheckforrequiredproperties: super::super::Foundation::BOOL, psznameofpropinerror: *mut windows_core::PWSTR) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_PROPERTY = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, poutpropertyitem: *mut *mut core::ffi::c_void, pcboutpropertyitemsize: *mut u32) -> u32>;
pub type PRESUTIL_GET_PROPERTY_FORMATS = Option<unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertyformatlist: *mut core::ffi::c_void, cbpropertyformatlistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_PROPERTY_SIZE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, pcboutpropertylistsize: *mut u32, pnpropertycount: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_QWORD_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, pqwoutvalue: *mut u64, qwdefaultvalue: u64) -> u32>;
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY = Option<unsafe extern "system" fn(hself: super::super::Foundation::HANDLE, lpszresourcetype: windows_core::PCWSTR) -> HRESOURCE>;
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_CLASS = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hself: super::super::Foundation::HANDLE, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: super::super::Foundation::BOOL) -> HRESOURCE>;
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_CLASS_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hself: super::super::Foundation::HANDLE, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HRESOURCE>;
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_NAME = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hself: super::super::Foundation::HANDLE, lpszresourcetype: windows_core::PCWSTR, brecurse: super::super::Foundation::BOOL) -> HRESOURCE>;
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_NAME_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hself: super::super::Foundation::HANDLE, lpszresourcetype: windows_core::PCWSTR, brecurse: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HRESOURCE>;
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_EX = Option<unsafe extern "system" fn(hself: super::super::Foundation::HANDLE, lpszresourcetype: windows_core::PCWSTR, dwdesiredaccess: u32) -> HRESOURCE>;
pub type PRESUTIL_GET_RESOURCE_DEPENDENTIP_ADDRESS_PROPS = Option<unsafe extern "system" fn(hresource: HRESOURCE, pszaddress: windows_core::PWSTR, pcchaddress: *mut u32, pszsubnetmask: windows_core::PWSTR, pcchsubnetmask: *mut u32, psznetwork: windows_core::PWSTR, pcchnetwork: *mut u32) -> u32>;
pub type PRESUTIL_GET_RESOURCE_NAME = Option<unsafe extern "system" fn(hresource: HRESOURCE, pszresourcename: windows_core::PWSTR, pcchresourcenameinout: *mut u32) -> u32>;
pub type PRESUTIL_GET_RESOURCE_NAME_DEPENDENCY = Option<unsafe extern "system" fn(lpszresourcename: windows_core::PCWSTR, lpszresourcetype: windows_core::PCWSTR) -> HRESOURCE>;
pub type PRESUTIL_GET_RESOURCE_NAME_DEPENDENCY_EX = Option<unsafe extern "system" fn(lpszresourcename: windows_core::PCWSTR, lpszresourcetype: windows_core::PCWSTR, dwdesiredaccess: u32) -> HRESOURCE>;
pub type PRESUTIL_GET_SZ_PROPERTY = Option<unsafe extern "system" fn(ppszoutvalue: *mut windows_core::PWSTR, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: windows_core::PCWSTR, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_SZ_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR) -> windows_core::PWSTR>;
pub type PRESUTIL_IS_PATH_VALID = Option<unsafe extern "system" fn(pszpath: windows_core::PCWSTR) -> super::super::Foundation::BOOL>;
pub type PRESUTIL_IS_RESOURCE_CLASS_EQUAL = Option<unsafe extern "system" fn(prci: *mut CLUS_RESOURCE_CLASS_INFO, hresource: HRESOURCE) -> super::super::Foundation::BOOL>;
pub type PRESUTIL_PROPERTY_LIST_FROM_PARAMETER_BLOCK = Option<unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut core::ffi::c_void, pcboutpropertylistsize: *mut u32, pinparams: *const u8, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
pub type PRESUTIL_REMOVE_RESOURCE_SERVICE_ENVIRONMENT = Option<unsafe extern "system" fn(pszservicename: windows_core::PCWSTR, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
pub type PRESUTIL_RESOURCES_EQUAL = Option<unsafe extern "system" fn(hself: HRESOURCE, hresource: HRESOURCE) -> super::super::Foundation::BOOL>;
pub type PRESUTIL_RESOURCE_TYPES_EQUAL = Option<unsafe extern "system" fn(lpszresourcetypename: windows_core::PCWSTR, hresource: HRESOURCE) -> super::super::Foundation::BOOL>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_BINARY_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, pbnewvalue: *const u8, cbnewvaluesize: u32, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_DWORD_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, dwnewvalue: u32, pdwoutvalue: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_EXPAND_SZ_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, psznewvalue: windows_core::PCWSTR, ppszoutstring: *mut windows_core::PWSTR) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_MULTI_SZ_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, psznewvalue: windows_core::PCWSTR, cbnewvaluesize: u32, ppszoutvalue: *mut windows_core::PWSTR, pcboutvaluesize: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_PRIVATE_PROPERTY_LIST = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_PROPERTY_PARAMETER_BLOCK = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_PROPERTY_PARAMETER_BLOCK_EX = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: super::super::Foundation::BOOL, poutparams: *mut u8) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_PROPERTY_TABLE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *const core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_PROPERTY_TABLE_EX = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: super::super::Foundation::BOOL, poutparams: *mut u8) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_QWORD_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, qwnewvalue: u64, pqwoutvalue: *mut u64) -> u32>;
pub type PRESUTIL_SET_RESOURCE_SERVICE_ENVIRONMENT = Option<unsafe extern "system" fn(pszservicename: windows_core::PCWSTR, hresource: HRESOURCE, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
#[cfg(feature = "Win32_Security")]
pub type PRESUTIL_SET_RESOURCE_SERVICE_START_PARAMETERS = Option<unsafe extern "system" fn(pszservicename: windows_core::PCWSTR, schscmhandle: super::super::Security::SC_HANDLE, phservice: *mut super::super::Security::SC_HANDLE, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
#[cfg(feature = "Win32_Security")]
pub type PRESUTIL_SET_RESOURCE_SERVICE_START_PARAMETERS_EX = Option<unsafe extern "system" fn(pszservicename: windows_core::PCWSTR, schscmhandle: super::super::Security::SC_HANDLE, phservice: *mut super::super::Security::SC_HANDLE, dwdesiredaccess: u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_SZ_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, psznewvalue: windows_core::PCWSTR, ppszoutstring: *mut windows_core::PWSTR) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_UNKNOWN_PROPERTIES = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32) -> u32>;
#[cfg(feature = "Win32_Security")]
pub type PRESUTIL_START_RESOURCE_SERVICE = Option<unsafe extern "system" fn(pszservicename: windows_core::PCWSTR, phservicehandle: *mut super::super::Security::SC_HANDLE) -> u32>;
pub type PRESUTIL_STOP_RESOURCE_SERVICE = Option<unsafe extern "system" fn(pszservicename: windows_core::PCWSTR) -> u32>;
#[cfg(feature = "Win32_Security")]
pub type PRESUTIL_STOP_SERVICE = Option<unsafe extern "system" fn(hservicehandle: super::super::Security::SC_HANDLE) -> u32>;
pub type PRESUTIL_TERMINATE_SERVICE_PROCESS_FROM_RES_DLL = Option<unsafe extern "system" fn(dwservicepid: u32, boffline: super::super::Foundation::BOOL, pdwresourcestate: *mut u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
pub type PRESUTIL_VERIFY_PRIVATE_PROPERTY_LIST = Option<unsafe extern "system" fn(pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32) -> u32>;
pub type PRESUTIL_VERIFY_PROPERTY_TABLE = Option<unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *const core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32>;
pub type PRESUTIL_VERIFY_RESOURCE_SERVICE = Option<unsafe extern "system" fn(pszservicename: windows_core::PCWSTR) -> u32>;
#[cfg(feature = "Win32_Security")]
pub type PRESUTIL_VERIFY_SERVICE = Option<unsafe extern "system" fn(hservicehandle: super::super::Security::SC_HANDLE) -> u32>;
pub type PRES_UTIL_VERIFY_SHUTDOWN_SAFE = Option<unsafe extern "system" fn(flags: u32, reason: u32, presult: *mut u32) -> u32>;
pub type PSET_INTERNAL_STATE = Option<unsafe extern "system" fn(param0: isize, statetype: CLUSTER_RESOURCE_APPLICATION_STATE, active: super::super::Foundation::BOOL) -> u32>;
pub type PSET_RESOURCE_INMEMORY_NODELOCAL_PROPERTIES_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, propertylistbuffer: *const u8, propertylistbuffersize: u32) -> u32>;
pub type PSET_RESOURCE_LOCKED_MODE_EX_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, lockedmodeenabled: super::super::Foundation::BOOL, lockedmodereason: u32, lockedmodeflags: u32) -> u32>;
pub type PSET_RESOURCE_LOCKED_MODE_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, lockedmodeenabled: super::super::Foundation::BOOL, lockedmodereason: u32) -> u32>;
pub type PSET_RESOURCE_STATUS_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, resourcestatus: *mut RESOURCE_STATUS) -> u32>;
pub type PSET_RESOURCE_STATUS_ROUTINE_EX = Option<unsafe extern "system" fn(resourcehandle: isize, resourcestatus: *mut RESOURCE_STATUS_EX) -> u32>;
pub type PSET_RESOURCE_WPR_POLICY_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, wprpolicyflags: u32) -> u32>;
pub type PSIGNAL_FAILURE_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, failuretype: FAILURE_TYPE, applicationspecificerrorcode: u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PSTARTUP_EX_ROUTINE = Option<unsafe extern "system" fn(resourcetype: windows_core::PCWSTR, minversionsupported: u32, maxversionsupported: u32, monitorcallbackfunctions: *mut CLRES_CALLBACK_FUNCTION_TABLE, resourcedllinterfacefunctions: *mut *mut CLRES_FUNCTION_TABLE) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PSTARTUP_ROUTINE = Option<unsafe extern "system" fn(resourcetype: windows_core::PCWSTR, minversionsupported: u32, maxversionsupported: u32, setresourcestatus: PSET_RESOURCE_STATUS_ROUTINE, logevent: PLOG_EVENT_ROUTINE, functiontable: *mut *mut CLRES_FUNCTION_TABLE) -> u32>;
pub type PTERMINATE_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void)>;
pub type PWORKER_START_ROUTINE = Option<unsafe extern "system" fn(pworker: *mut CLUS_WORKER, lpthreadparameter: *mut core::ffi::c_void) -> u32>;
pub type SET_APP_INSTANCE_CSV_FLAGS = Option<unsafe extern "system" fn(processhandle: super::super::Foundation::HANDLE, mask: u32, flags: u32) -> u32>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
