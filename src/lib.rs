use std::{ffi::c_void, ptr::null_mut};
use windows::{core::PCWSTR, Win32::{Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, HANDLE}, System::{SystemServices::*, Threading::CreateMutexW}, UI::WindowsAndMessaging::{MessageBoxA, MB_OK}}};
use windows::core::{s, w};
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::LibraryLoader::FreeLibraryAndExitThread;
use windows::Win32::System::Threading::{CreateThread, LPTHREAD_START_ROUTINE, THREAD_CREATION_FLAGS};

static mut HMODULE_INSTANCE: HINSTANCE = HINSTANCE(null_mut()); // handle to the module instance of the injected dll

enum LoadModule {
    FreeLibrary,
    StartImplant,
}

struct Implant {
    mutex_handle: HANDLE,
    mutex_name: PCWSTR,
}

impl Default for Implant {
    fn default() -> Implant {
        Implant {
            mutex_handle: HANDLE(null_mut()),
            mutex_name: w!("MyImplantMutex"),
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
fn DllMain(hmod_instance: HINSTANCE, dw_reason: u32, _: usize) -> i32 {
    match dw_reason {
        DLL_PROCESS_ATTACH => unsafe {
            HMODULE_INSTANCE = hmod_instance; // set a handle to the module for a clean unload
            spawn_thread(LoadModule::StartImplant); // start implant in a new thread
        },
        _ => (),
    }

    1
}

/// Entrypoint to the actual implant to be spawned as a new thread from DLL_PROCESS_ATTACH.
/// This should help to prevent problems whereby a LoaderLock interferes with our implant.<br/><br/>
/// Think of this as calling a function to start something from main().
#[no_mangle]
unsafe extern "system" fn attach(_lp_thread_param: *mut c_void) -> u32 {

    // initialise the implant
    let mut implant = Implant::default();

    //
    // First thing we should do is create the mutex to ensure only ONE instance of our payload is running
    // if the result of creating the mutex is none, exit the thread.
    //
    // This function will take care of moving the handle to the mutex into implant, so when we unload, we can
    // clean up the handle & delete the mutex
    //
    if create_global_mutex(&mut implant).is_none() { return 0 };

    MessageBoxA(None, s!("Implant injected :E"), s!("Implant injected :E"), MB_OK);

    // clean up the implant now our work is done
    cleanup_implant(&implant);

    1
}

/// Spawn a new thread in the current injected process, calling a function pointer to a function
/// will run.
fn spawn_thread(lib_to_load: LoadModule) {
    unsafe {
        // function pointer to where the new thread will begin
        let thread_start: LPTHREAD_START_ROUTINE;

        match lib_to_load {
            LoadModule::FreeLibrary => thread_start = Some(unload_dll),
            LoadModule::StartImplant => thread_start = Some(attach)
        }

        // create a thread with a function pointer to the region of the program we want to execute.
        let _thread_handle = CreateThread(
            None,
            0,
            thread_start,
            None,
            THREAD_CREATION_FLAGS(0),
            None,
        );
    }
}

#[no_mangle]
/// Unload the DLL by its handle, so that there is no live evidence of hte DLL in memory after its
/// finished its business, plus allows for loading multiple of the same DLL into the same process
unsafe extern "system" fn unload_dll(_lpthread_param: *mut c_void) -> u32 {
    MessageBoxA(None, s!("Unloading"), s!("Unloading"), MB_OK);
    FreeLibraryAndExitThread(HMODULE_INSTANCE, 1);
}

/// Create a wide char global mutex on Windows which will prevent multiple
/// instances of this DLL being loaded into a process -we only want 1 
/// instance of our implant running on a machine :)
fn create_global_mutex(implant: &mut Implant) -> Option<()> {
    let result = unsafe {
        CreateMutexW(None, true, implant.mutex_name)
    };

    let result: Option<()> = match result {
        Ok(h) => {

            let last_error = unsafe { GetLastError() };
            if last_error == ERROR_ALREADY_EXISTS {
                eprintln!("[-] Mutex exists: {}", last_error.0);
                return None;
            }

            implant.mutex_handle = h;

            Some(())
        }, 
        Err(e) => {
            eprintln!("[-] Error: {}", e);
            return None;
        },
    };

    result
}

/// Cleanup the implant, ensuring handles and any other environment info 
/// is cleaned up
fn cleanup_implant(implant: &Implant) {
    unsafe {
        CloseHandle(implant.mutex_handle);
    };
}